use std::collections::HashMap;
use std::path::PathBuf;
use std::sync::Arc;
use std::sync::atomic::{AtomicUsize, Ordering};

use anyhow::Context as _;
use futures::StreamExt as _;
use rho_agent::Agent;
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
    next_agent_id: AtomicUsize,
}

impl AgentRegistry {
    async fn new(db: RhoDb, auth: InferenceAuth, inference_config: InferenceConfig) -> Self {
        let initial_agent_id = "agent-1".to_owned();
        let initial_agent = Agent::create(
            db.clone(),
            auth.clone(),
            inference_config.clone(),
            Some(initial_agent_id.clone()),
        )
        .await;
        let mut agents = HashMap::new();
        agents.insert(initial_agent_id, initial_agent);
        Self {
            db,
            auth,
            inference_config,
            agents: Mutex::new(agents),
            next_agent_id: AtomicUsize::new(2),
        }
    }

    async fn list(&self) -> Vec<(String, Agent)> {
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

    async fn create(&self, requested_agent_id: Option<String>) -> (String, Agent, bool) {
        let agent_id = requested_agent_id.unwrap_or_else(|| {
            format!(
                "agent-{}",
                self.next_agent_id.fetch_add(1, Ordering::Relaxed)
            )
        });
        if let Some(agent) = self.agents.lock().await.get(&agent_id).cloned() {
            return (agent_id, agent.clone(), false);
        }
        let agent = Agent::create(
            self.db.clone(),
            self.auth.clone(),
            self.inference_config.clone(),
            Some(agent_id.clone()),
        )
        .await;
        let mut agents = self.agents.lock().await;
        if let Some(agent) = agents.get(&agent_id) {
            return (agent_id, agent.clone(), false);
        }
        agents.insert(agent_id.clone(), agent.clone());
        (agent_id, agent, true)
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

    for (agent_id, agent) in agents.list().await {
        subscribe_agent(agent_id, agent, outgoing_tx.clone());
    }

    let mut reader = reader;
    loop {
        match read_frame_counted::<_, ClientMessage>(&mut reader, Some(&counters)).await? {
            ClientMessage::Ping => {
                let _ = outgoing_tx.send(ServerMessage::Pong);
            }
            ClientMessage::Subscribe => {}
            ClientMessage::CreateAgent { agent_id } => {
                let (agent_id, agent, created) = agents.create(Some(agent_id)).await;
                if created {
                    subscribe_agent(agent_id.clone(), agent, outgoing_tx.clone());
                }
                let _ = outgoing_tx.send(ServerMessage::AgentCreated { agent_id });
            }
            ClientMessage::SendUserMessage { agent_id, content } => {
                let agent = match agents.get(&agent_id).await {
                    Some(agent) => agent,
                    None => {
                        let (_, agent, _) = agents.create(Some(agent_id.clone())).await;
                        subscribe_agent(agent_id.clone(), agent.clone(), outgoing_tx.clone());
                        agent
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
