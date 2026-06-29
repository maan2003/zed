use std::collections::HashMap;
use std::path::PathBuf;
use std::str::FromStr as _;
use std::sync::Arc;
use std::sync::atomic::{AtomicUsize, Ordering};

use anyhow::Context as _;
use futures::StreamExt as _;
use rho_agent::Agent;
use rho_agent::db::{AgentId, AgentReadTxnExt as _, AgentWriteTxnExt as _};
use rho_core::text_content;
use rho_db::RhoDb;
use rho_inference::InferenceAuth;
use rho_inference::config::InferenceConfig;
use rho_ui_proto::remote::AgentRemoteEncoder;
use rho_ui_proto::server::{Server, ServerConnection};
use rho_ui_proto::{ClientMessage, ServerMessage, read_frame_counted, write_frame_counted};
use tokio::sync::{Mutex, Notify, mpsc};

pub fn default_socket_path() -> anyhow::Result<PathBuf> {
    let base = dirs::runtime_dir()
        .or_else(dirs::state_dir)
        .ok_or_else(|| anyhow::anyhow!("runtime directory not available"))?;
    Ok(base.join("rho").join("rho.sock"))
}

pub fn default_db_path() -> anyhow::Result<PathBuf> {
    let base = dirs::state_dir().ok_or_else(|| anyhow::anyhow!("state directory not available"))?;
    Ok(base.join("rho").join("rho.redb"))
}

#[derive(Clone, Debug, clap::Args)]
pub struct DaemonArgs {
    #[arg(long = "auth", default_value = "default")]
    pub auth: String,
    #[arg(long = "socket-path")]
    pub socket_path: Option<PathBuf>,
    /// Exit once the last UI client disconnects.
    #[arg(long = "die-on-detached")]
    pub die_on_detached: bool,
}

pub async fn run(args: DaemonArgs) -> anyhow::Result<()> {
    let socket_path = args.socket_path.unwrap_or(default_socket_path()?);
    if let Some(parent) = socket_path.parent() {
        std::fs::create_dir_all(parent).context("create socket directory")?;
    }
    let _ = std::fs::remove_file(&socket_path);
    let server = Server::bind(&socket_path).context("bind rho daemon socket")?;

    let db = RhoDb::open(default_db_path()?);
    let auth = InferenceAuth::named(&args.auth)?;
    let inference_config = InferenceConfig::deep();
    let agents = Arc::new(AgentRegistry::new(db, auth, inference_config).await);

    let active_connections = Arc::new(AtomicUsize::new(0));
    let connection_closed = Arc::new(Notify::new());
    let mut accepted_connection = false;

    loop {
        if args.die_on_detached
            && accepted_connection
            && active_connections.load(Ordering::Relaxed) == 0
        {
            return Ok(());
        }

        tokio::select! {
            connection = server.accept() => {
                let connection = connection?;
                accepted_connection = true;
                active_connections.fetch_add(1, Ordering::Relaxed);
                let agents = agents.clone();
                let active_connections = active_connections.clone();
                let connection_closed = connection_closed.clone();
                tokio::spawn(async move {
                    if let Err(error) = serve_connection(agents, connection).await {
                        eprintln!("rho daemon connection error: {error:#}");
                    }
                    active_connections.fetch_sub(1, Ordering::Relaxed);
                    connection_closed.notify_one();
                });
            }
            () = connection_closed.notified(), if active_connections.load(Ordering::Relaxed) > 0 => {}
        }
    }
}

struct AgentRegistry {
    db: RhoDb,
    auth: InferenceAuth,
    inference_config: InferenceConfig,
    agents: Mutex<HashMap<String, Agent>>,
    known_agent_ids: Mutex<Vec<String>>,
}

impl AgentRegistry {
    async fn new(db: RhoDb, auth: InferenceAuth, inference_config: InferenceConfig) -> Self {
        let mut write = db.write().await;
        write.init_agent_tables();
        write.commit();
        let mut known_agent_ids = db
            .read()
            .list_agents()
            .into_iter()
            .map(|(agent_id, _)| agent_id.to_string())
            .collect::<Vec<_>>();
        known_agent_ids.sort();
        Self {
            db,
            auth,
            inference_config,
            agents: Mutex::new(HashMap::new()),
            known_agent_ids: Mutex::new(known_agent_ids),
        }
    }

    async fn known_agent_ids(&self) -> Vec<String> {
        self.known_agent_ids.lock().await.clone()
    }

    async fn remember_agent_id(&self, agent_id: String) {
        let mut known_agent_ids = self.known_agent_ids.lock().await;
        if !known_agent_ids.contains(&agent_id) {
            known_agent_ids.push(agent_id);
            known_agent_ids.sort();
        }
    }

    async fn loaded(&self) -> Vec<(String, Agent)> {
        let mut agents = self
            .agents
            .lock()
            .await
            .iter()
            .map(|(agent_id, agent)| (agent_id.clone(), agent.clone()))
            .collect::<Vec<_>>();
        agents.sort_by(|(left, _), (right, _)| left.cmp(right));
        agents
    }

    async fn get(&self, agent_id: &str) -> Option<Agent> {
        self.agents.lock().await.get(agent_id).cloned()
    }

    async fn create(&self) -> (String, Agent) {
        let (agent_id, agent) = Agent::create_with_id(
            self.db.clone(),
            self.auth.clone(),
            self.inference_config.clone(),
            None,
        )
        .await;
        let agent_id = agent_id.to_string();
        self.agents
            .lock()
            .await
            .insert(agent_id.clone(), agent.clone());
        self.remember_agent_id(agent_id.clone()).await;
        (agent_id, agent)
    }

    async fn load(&self, agent_id: &str) -> anyhow::Result<(String, Agent, bool)> {
        if let Some(agent) = self.agents.lock().await.get(agent_id).cloned() {
            return Ok((agent_id.to_owned(), agent, false));
        }
        let parsed = AgentId::from_str(agent_id)
            .map_err(|_| anyhow::anyhow!("invalid agent id: {agent_id}"))?;
        let canonical_agent_id = parsed.to_string();
        if !self
            .known_agent_ids
            .lock()
            .await
            .contains(&canonical_agent_id)
        {
            anyhow::bail!("unknown agent id: {agent_id}");
        }
        let agent = Agent::load(self.db.clone(), self.auth.clone(), parsed);
        self.agents
            .lock()
            .await
            .insert(canonical_agent_id.clone(), agent.clone());
        Ok((canonical_agent_id, agent, true))
    }
}

async fn serve_connection(
    agents: Arc<AgentRegistry>,
    connection: ServerConnection,
) -> anyhow::Result<()> {
    let counters = connection.io_counters();
    let stream = connection.into_stream();
    let (reader, writer) = stream.into_split();

    let (outgoing_tx, mut outgoing_rx) = mpsc::unbounded_channel::<ServerMessage>();
    let writer_counters = counters.clone();
    tokio::spawn(async move {
        let mut writer = writer;
        while let Some(message) = outgoing_rx.recv().await {
            if write_frame_counted(&mut writer, &message, Some(&writer_counters))
                .await
                .is_err()
            {
                break;
            }
        }
    });

    let _ = outgoing_tx.send(ServerMessage::Ready {
        agent_ids: agents.known_agent_ids().await,
    });

    for (agent_id, agent) in agents.loaded().await {
        subscribe_agent(agent_id, agent, outgoing_tx.clone());
    }

    let mut reader = reader;
    loop {
        match read_frame_counted::<_, ClientMessage>(&mut reader, Some(&counters)).await? {
            ClientMessage::Ping => {
                let _ = outgoing_tx.send(ServerMessage::Pong);
            }
            ClientMessage::Subscribe => {}
            ClientMessage::NewAgent { content } => {
                let (agent_id, agent) = agents.create().await;
                subscribe_agent(agent_id.clone(), agent.clone(), outgoing_tx.clone());
                let _ = outgoing_tx.send(ServerMessage::AgentCreated {
                    agent_id: agent_id.clone(),
                });
                let _ = outgoing_tx.send(ServerMessage::Ready {
                    agent_ids: agents.known_agent_ids().await,
                });
                if let Some(content) = content {
                    agent.send_user_message(text_content(&content));
                }
            }
            ClientMessage::LoadAgent { agent_id } => match agents.load(&agent_id).await {
                Ok((agent_id, agent, loaded_now)) => {
                    if loaded_now {
                        subscribe_agent(agent_id.clone(), agent, outgoing_tx.clone());
                    }
                    let _ = outgoing_tx.send(ServerMessage::AgentLoaded { agent_id });
                }
                Err(error) => {
                    let _ = outgoing_tx.send(ServerMessage::Error {
                        message: error.to_string(),
                    });
                }
            },
            ClientMessage::SendUserMessage { agent_id, content } => {
                let agent = match agents.get(&agent_id).await {
                    Some(agent) => agent,
                    None => {
                        let _ = outgoing_tx.send(ServerMessage::Error {
                            message: format!("agent is not loaded: {agent_id}"),
                        });
                        continue;
                    }
                };
                agent.send_user_message(text_content(&content));
            }
            ClientMessage::CancelTurn { agent_id } => {
                if let Some(agent) = agents.get(&agent_id).await {
                    agent.cancel();
                    let _ = outgoing_tx.send(ServerMessage::TurnCancelled { agent_id });
                }
            }
        }
    }
}

fn subscribe_agent(agent_id: String, agent: Agent, state_tx: mpsc::UnboundedSender<ServerMessage>) {
    tokio::spawn(async move {
        let changes = agent.subscribe();
        let mut encoder = AgentRemoteEncoder::new();
        let _ = state_tx.send(ServerMessage::Agent {
            agent_id: agent_id.clone(),
            frame: encoder.encode(agent.state()),
        });
        futures::pin_mut!(changes);
        while let Some(state) = changes.next().await {
            if state_tx
                .send(ServerMessage::Agent {
                    agent_id: agent_id.clone(),
                    frame: encoder.encode(state),
                })
                .is_err()
            {
                break;
            }
        }
    });
}
