//! The harness-extension run loop.
//!
//! This is where the durable model meets the live harness. The factory speaks
//! the tau extension protocol: it publishes human-facing `/factory` actions,
//! subscribes to agent lifecycle events, and turns a `start` action into a jj
//! workspace plus a `UiCreateAgent` request. The single new capability the
//! factory needs — spawning an agent bound to a workspace — is composed from
//! primitives that already exist: agent creation carries the workspace as the
//! shell's `cwd` metadata, and the task binding rides back to us on
//! `AgentStarted` as our own `factory_task_id` metadata.
//!
//! The loop mirrors the other tau extensions: a synchronous read/write loop
//! over the protocol's framed messages. The one async call (creating a jj
//! workspace) is bridged with `smol::block_on`, because the rest of the loop is
//! blocking I/O.

use std::error::Error;
use std::io::{BufReader, BufWriter, Read, Write};
use std::path::{Path, PathBuf};

use tau_actions::{ActionArg, ActionArgKind, ActionCommand, ActionSchema};
use tau_proto::{
    ActionError, ActionInvoke, ActionOutput, ActionResult, AgentInitialMetadata, AgentMetadataKey,
    AgentStarted, CborValue, ConfigError, Configure, CustomEvent, Event, EventCategory, EventName,
    HarnessInputMessage, HarnessOutputMessage, PeerInputReader, PeerOutputWriter, UiCreateAgent,
};

use crate::store::TaskStore;
use crate::task::wire::{
    CATEGORY as FACTORY_CATEGORY, PROJECTS_UPDATE as PROJECTS_UPDATE_CALL, SYNC as SYNC_CALL,
    TASKS_UPDATE as TASKS_UPDATE_CALL, TOPICS_UPDATE as TOPICS_UPDATE_CALL,
};
use crate::task::{Project, Status, Task, TaskId, Topic, TopicId};

/// Role bound to agents the factory spawns. Matches the role the CLI uses for
/// interactive agents, so factory agents get the same tools and prompt.
const ROLE: &str = "senior-engineer";
/// Metadata key the shell extension reads to set an agent's working directory.
/// Shaped `ext_{instance}_cwd` for the default `core-shell` instance.
const SHELL_CWD_KEY: &str = "ext_core-shell_cwd";
/// Metadata key binding a spawned agent back to its task. The harness echoes it
/// on `AgentStarted`, which is how we learn the new agent's id for a task.
const FACTORY_TASK_ID_KEY: &str = "ext_factory_task-id";
const FACTORY_TOPIC_ID_KEY: &str = "ext_factory_topic-id";

pub fn run_stdio() -> Result<(), Box<dyn Error>> {
    run(std::io::stdin(), std::io::stdout())
}

pub fn run<R, W>(reader: R, writer: W) -> Result<(), Box<dyn Error>>
where
    R: Read,
    W: Write,
{
    let mut reader = PeerInputReader::new(BufReader::new(reader));
    let mut writer = PeerOutputWriter::new(BufWriter::new(writer));

    tau_extension::Handshake::tool("tau-factory")
        .subscribe([
            EventName::ACTION_INVOKE,
            EventName::AGENT_STARTED,
            factory_event(SYNC_CALL),
        ])
        .publish_actions(factory_action_schema())
        .ready_message("factory ready")
        .run(&mut writer)?;

    // `None` until the harness sends `Configure`, which carries the state
    // directory the store lives in. Deliveries before that are ignored.
    let mut factory: Option<Factory> = None;

    while let Some(message) = reader.read_message()? {
        match message {
            HarnessOutputMessage::Configure(message) => match Factory::configure(&message) {
                Ok(configured) => factory = Some(configured),
                Err(message) => {
                    writer.write_message(&HarnessInputMessage::ConfigError(ConfigError {
                        message,
                    }))?;
                    writer.flush()?;
                }
            },
            HarnessOutputMessage::Deliver(delivery) => {
                let is_replay = delivery.is_replay();
                let Some(factory) = factory.as_mut() else {
                    continue;
                };
                match delivery.into_event() {
                    // Invoking an action is an execution trigger; replayed
                    // history must not re-create agents.
                    Event::ActionInvoke(invoke) if !is_replay => {
                        match factory.handle_action(invoke, &mut writer)? {
                            Some(FactoryActionChange::Task(task)) => {
                                factory.emit_tasks_update(vec![task], &mut writer)?;
                            }
                            Some(FactoryActionChange::Projects) => {
                                factory.emit_all_projects(&mut writer)?;
                            }
                            Some(FactoryActionChange::Topics(topics)) => {
                                factory.emit_topics_update(topics, &mut writer)?;
                            }
                            None => {}
                        }
                    }
                    // Binding a task to its agent is idempotent (it only fires
                    // while the task is still `Open`), so it is safe — and useful
                    // for recovery — to fold replayed `AgentStarted` too.
                    Event::AgentStarted(started) => {
                        if let Some(task) = factory.handle_agent_started(started) {
                            factory.emit_tasks_update(vec![task], &mut writer)?;
                        }
                        if let Some(topics) = factory.take_pending_topic_update() {
                            factory.emit_topics_update(topics, &mut writer)?;
                        }
                    }
                    // A UI asks for the current board. Custom events are not
                    // replayed, so this request is the only way a freshly
                    // connected client learns the tasks that already exist — the
                    // one case where we send the whole list rather than a delta.
                    Event::ExtensionEvent(custom) if is_factory_event(custom.name(), SYNC_CALL) => {
                        let all = factory.store.list().collect();
                        factory.emit_tasks_update(all, &mut writer)?;
                        factory.emit_all_projects(&mut writer)?;
                        factory.emit_all_topics(&mut writer)?;
                    }
                    _ => {}
                }
            }
            HarnessOutputMessage::Disconnect(_) => break,
            _ => {}
        }
    }

    Ok(())
}

/// Per-extension config from `harness.yaml`. The project's repository is the one
/// thing the harness does not hand us directly, so it comes from config (falling
/// back to the process working directory).
#[derive(Debug, Default, serde::Deserialize)]
#[serde(default, deny_unknown_fields)]
struct FactoryConfig {
    repo_root: Option<PathBuf>,
}

struct Factory {
    store: TaskStore,
    /// Repository the per-task workspaces share.
    repo_root: PathBuf,
    /// Directory the per-task workspaces are created under.
    workspaces_dir: PathBuf,
    pending_topic_update: Option<Vec<Topic>>,
}

enum FactoryActionChange {
    Task(Task),
    Projects,
    Topics(Vec<Topic>),
}

impl Factory {
    fn configure(message: &Configure) -> Result<Self, String> {
        let state_dir = message
            .state_dir
            .clone()
            .ok_or("factory requires a state directory")?;
        let config: FactoryConfig = tau_extension::parse_config(&message.config)?;
        let repo_root = config
            .repo_root
            .or_else(|| std::env::current_dir().ok())
            .ok_or("could not determine the project repository root")?;

        // The store file and the workspace parent must exist before they are
        // used: `Database::create` does not create missing parent directories,
        // and `jj workspace add` needs the destination's parent to exist.
        let workspaces_dir = state_dir.join("workspaces");
        std::fs::create_dir_all(&workspaces_dir)
            .map_err(|error| format!("preparing factory state directory: {error}"))?;

        Ok(Self {
            store: TaskStore::open(&state_dir.join("factory.redb")),
            repo_root,
            workspaces_dir,
            pending_topic_update: None,
        })
    }

    /// Handles a slash action and returns what changed, if anything, so the
    /// caller can emit the matching update event. `start` creates a workspace
    /// and requests an agent but leaves the task `Open`, so it does not itself
    /// change the board — the `Active` flip rides back on `AgentStarted`.
    fn handle_action<O: Write>(
        &mut self,
        invoke: ActionInvoke,
        writer: &mut PeerOutputWriter<O>,
    ) -> Result<Option<FactoryActionChange>, Box<dyn Error>> {
        match invoke.action_id.as_str() {
            "new" => match self.action_new(&invoke) {
                Ok(task) => {
                    self.reply(&invoke, Ok(format!("created {}", task.id)), writer)?;
                    Ok(Some(FactoryActionChange::Task(task)))
                }
                Err(message) => {
                    self.reply(&invoke, Err(message), writer)?;
                    Ok(None)
                }
            },
            "project_add" => match self.action_project_add(&invoke) {
                Ok(project) => {
                    self.reply(&invoke, Ok(format!("added {}", project.name)), writer)?;
                    Ok(Some(FactoryActionChange::Projects))
                }
                Err(message) => {
                    self.reply(&invoke, Err(message), writer)?;
                    Ok(None)
                }
            },
            "topic_new" => match self.action_topic_new(&invoke) {
                Ok(topic) => {
                    self.reply(&invoke, Ok(format!("created {}", topic.id)), writer)?;
                    Ok(Some(FactoryActionChange::Topics(vec![topic])))
                }
                Err(message) => {
                    self.reply(&invoke, Err(message), writer)?;
                    Ok(None)
                }
            },
            "topic_rename" => match self.action_topic_rename(&invoke) {
                Ok(topic) => {
                    self.reply(&invoke, Ok(format!("renamed {}", topic.id)), writer)?;
                    Ok(Some(FactoryActionChange::Topics(vec![topic])))
                }
                Err(message) => {
                    self.reply(&invoke, Err(message), writer)?;
                    Ok(None)
                }
            },
            "topic_archive" => match self.action_topic_archive(&invoke) {
                Ok(topic) => {
                    self.reply(&invoke, Ok(format!("archived {}", topic.id)), writer)?;
                    Ok(Some(FactoryActionChange::Topics(vec![topic])))
                }
                Err(message) => {
                    self.reply(&invoke, Err(message), writer)?;
                    Ok(None)
                }
            },
            "start" => {
                match self.action_start(&invoke) {
                    Ok((create_agent, reply)) => {
                        writer.write_message(&HarnessInputMessage::emit(create_agent))?;
                        self.reply(&invoke, Ok(reply), writer)?;
                    }
                    Err(message) => self.reply(&invoke, Err(message), writer)?,
                }
                Ok(None)
            }
            other => {
                self.reply(&invoke, Err(format!("unknown action `{other}`")), writer)?;
                Ok(None)
            }
        }
    }

    /// Emits a `factory.tasks_update` carrying `tasks`. Each entry is an upsert
    /// keyed by `TaskId`: callers pass just the task(s) that changed, or the
    /// whole list in response to a `factory.sync`. The factory imposes no
    /// ordering — the board sorts and groups at render time.
    fn emit_tasks_update<O: Write>(
        &self,
        tasks: Vec<Task>,
        writer: &mut PeerOutputWriter<O>,
    ) -> Result<(), Box<dyn Error>> {
        let payload = CborValue::serialized(&tasks)?;
        let event = CustomEvent::try_new(factory_event(TASKS_UPDATE_CALL), payload)?;
        writer.write_message(&HarnessInputMessage::emit(Event::ExtensionEvent(event)))?;
        writer.flush()?;
        Ok(())
    }

    fn emit_projects_update<O: Write>(
        &self,
        projects: Vec<Project>,
        writer: &mut PeerOutputWriter<O>,
    ) -> Result<(), Box<dyn Error>> {
        let payload = CborValue::serialized(&projects)?;
        let event = CustomEvent::try_new(factory_event(PROJECTS_UPDATE_CALL), payload)?;
        writer.write_message(&HarnessInputMessage::emit(Event::ExtensionEvent(event)))?;
        writer.flush()?;
        Ok(())
    }

    fn emit_topics_update<O: Write>(
        &self,
        topics: Vec<Topic>,
        writer: &mut PeerOutputWriter<O>,
    ) -> Result<(), Box<dyn Error>> {
        let payload = CborValue::serialized(&topics)?;
        let event = CustomEvent::try_new(factory_event(TOPICS_UPDATE_CALL), payload)?;
        writer.write_message(&HarnessInputMessage::emit(Event::ExtensionEvent(event)))?;
        writer.flush()?;
        Ok(())
    }

    fn emit_all_projects<O: Write>(
        &self,
        writer: &mut PeerOutputWriter<O>,
    ) -> Result<(), Box<dyn Error>> {
        self.emit_projects_update(self.store.list_projects().collect(), writer)
    }

    fn emit_all_topics<O: Write>(
        &self,
        writer: &mut PeerOutputWriter<O>,
    ) -> Result<(), Box<dyn Error>> {
        self.emit_topics_update(self.store.list_topics().collect(), writer)
    }

    fn action_new(&mut self, invoke: &ActionInvoke) -> Result<Task, String> {
        let title = invoke.argv.join(" ").trim().to_owned();
        if title.is_empty() {
            return Err("usage: /factory new <title>".to_owned());
        }
        Ok(self.store.create(title, String::new()))
    }

    fn action_project_add(&mut self, invoke: &ActionInvoke) -> Result<Project, String> {
        let raw_path = invoke.argv.join(" ").trim().to_owned();
        if raw_path.is_empty() {
            return Err("usage: /project add <path>".to_owned());
        }
        let path = PathBuf::from(raw_path);
        let name = project_name_from_path(&path)?;
        Ok(self.store.create_project(name, path))
    }

    fn action_topic_new(&mut self, invoke: &ActionInvoke) -> Result<Topic, String> {
        let name = invoke.argv.join(" ").trim().to_owned();
        if name.is_empty() {
            return Err("usage: /topic new <name>".to_owned());
        }
        let topic = self.store.create_topic(name);
        Ok(topic)
    }

    fn action_topic_rename(&mut self, invoke: &ActionInvoke) -> Result<Topic, String> {
        let (topic_ref, name) =
            parse_topic_ref_and_rest(invoke, "usage: /topic rename <topic> <name>")?;
        if name.is_empty() {
            return Err("usage: /topic rename <topic> <name>".to_owned());
        }
        let mut topic = self.resolve_topic(topic_ref)?;
        topic.name = name;
        topic.updated_at = chrono::Utc::now();
        self.store.put_topic(topic.clone());
        Ok(topic)
    }

    fn action_topic_archive(&mut self, invoke: &ActionInvoke) -> Result<Topic, String> {
        let mut topic =
            self.resolve_topic_from_args(invoke, "usage: /topic archive <id-or-name>")?;
        topic.archived = true;
        topic.updated_at = chrono::Utc::now();
        self.store.put_topic(topic.clone());
        Ok(topic)
    }

    /// Creates the workspace and returns the `UiCreateAgent` event to emit plus
    /// the human-facing reply. The task stays `Open` here; it flips to `Active`
    /// once the agent actually starts and `AgentStarted` carries its id back.
    fn action_start(&mut self, invoke: &ActionInvoke) -> Result<(Event, String), String> {
        let id = parse_task_id(invoke)?;
        let task = self
            .store
            .get(id)
            .ok_or_else(|| format!("no such task {id}"))?;
        if !matches!(task.status, Status::Open) {
            return Err(format!("{id} is not open"));
        }

        let workspace = self.workspace_path(id);
        let name = format!("task-{}", id.0);
        smol::block_on(crate::workspace::create_workspace(
            &self.repo_root,
            &name,
            &workspace,
            None,
        ))
        .map_err(|error| format!("creating workspace for {id}: {error}"))?;

        let create_agent = Event::UiCreateAgent(UiCreateAgent {
            role: ROLE.to_owned(),
            model_override: None,
            metadata: vec![
                AgentInitialMetadata {
                    key: AgentMetadataKey::new(SHELL_CWD_KEY),
                    value: CborValue::Text(workspace.display().to_string()),
                    inheritable: true,
                },
                AgentInitialMetadata {
                    key: AgentMetadataKey::new(FACTORY_TASK_ID_KEY),
                    value: CborValue::Text(id.0.to_string()),
                    inheritable: true,
                },
            ],
            initial_prompt: Some(task.title),
            message_class: Default::default(),
            originator: Default::default(),
            ctx_id: None,
            parent_agent: None,
        });
        Ok((
            create_agent,
            format!("starting {id} in {}", workspace.display()),
        ))
    }

    /// Binds a spawned agent to its task, flipping it to `Active`. Returns the
    /// changed task when it bound, so the caller can emit a `tasks_update`.
    fn handle_agent_started(&mut self, started: AgentStarted) -> Option<Task> {
        self.pending_topic_update = Some(self.assign_started_agent_to_topic(&started));
        let id = factory_task_id(&started.metadata)?; // not an agent we spawned
        let mut task = self.store.get(id)?; // metadata names a task we do not know
        // Only bind while still `Open`; a replayed or duplicate `AgentStarted`
        // for an already-active task is a no-op.
        if !matches!(task.status, Status::Open) {
            return None;
        }
        if task
            .start(self.workspace_path(id), started.agent_id)
            .is_ok()
        {
            task.updated_at = chrono::Utc::now();
            self.store.put(task.clone());
            return Some(task);
        }
        None
    }

    fn assign_started_agent_to_topic(&mut self, started: &AgentStarted) -> Vec<Topic> {
        let topic_id = factory_topic_id(&started.metadata)
            .or_else(|| {
                started
                    .parent_agent
                    .as_ref()
                    .and_then(|agent_id| self.store.topic_id_for_agent(agent_id))
            })
            .unwrap_or_else(|| TopicId(crate::store::DEFAULT_TOPIC_ID.to_owned()));
        self.store
            .assign_agent_to_topic(started.agent_id.clone(), topic_id)
    }

    fn take_pending_topic_update(&mut self) -> Option<Vec<Topic>> {
        self.pending_topic_update
            .take()
            .filter(|topics| !topics.is_empty())
    }

    fn resolve_topic_from_args(
        &self,
        invoke: &ActionInvoke,
        usage: &'static str,
    ) -> Result<Topic, String> {
        let raw = invoke.argv.join(" ").trim().to_owned();
        if raw.is_empty() {
            return Err(usage.to_owned());
        }
        self.store
            .list_topics()
            .find(|topic| topic.id.0 == raw || topic.name == raw)
            .ok_or_else(|| format!("no such topic `{raw}`"))
    }

    fn resolve_topic(&self, raw: String) -> Result<Topic, String> {
        self.store
            .list_topics()
            .find(|topic| topic.id.0 == raw || topic.name == raw)
            .ok_or_else(|| format!("no such topic `{raw}`"))
    }

    fn reply<O: Write>(
        &self,
        invoke: &ActionInvoke,
        result: Result<String, String>,
        writer: &mut PeerOutputWriter<O>,
    ) -> Result<(), Box<dyn Error>> {
        let event = match result {
            Ok(text) => Event::ActionResult(ActionResult {
                invocation_id: invoke.invocation_id.clone(),
                action_id: invoke.action_id.clone(),
                output: ActionOutput::Text { text },
            }),
            Err(message) => Event::ActionError(ActionError {
                invocation_id: invoke.invocation_id.clone(),
                action_id: invoke.action_id.clone(),
                message,
                details: None,
            }),
        };
        writer.write_message(&HarnessInputMessage::emit(event))?;
        writer.flush()?;
        Ok(())
    }

    fn workspace_path(&self, id: TaskId) -> PathBuf {
        self.workspaces_dir.join(format!("task-{}", id.0))
    }
}

/// Builds one of the factory's custom event names (`factory.<call>`).
fn factory_event(call: &'static str) -> EventName {
    EventName::new(EventCategory::Other(FACTORY_CATEGORY.to_owned()), call)
}

/// True when `name` is the factory custom event `factory.<call>`.
fn is_factory_event(name: &EventName, call: &str) -> bool {
    name.category().as_str() == FACTORY_CATEGORY && name.call().as_str() == call
}

/// Reads the `factory_task_id` an agent was created with, identifying its task.
fn factory_task_id(metadata: &[AgentInitialMetadata]) -> Option<TaskId> {
    metadata
        .iter()
        .find(|item| item.key.as_str() == FACTORY_TASK_ID_KEY)
        .and_then(|item| match &item.value {
            CborValue::Text(value) => value.parse::<u64>().ok(),
            _ => None,
        })
        .map(TaskId)
}

fn factory_topic_id(metadata: &[AgentInitialMetadata]) -> Option<TopicId> {
    metadata
        .iter()
        .find(|item| item.key.as_str() == FACTORY_TOPIC_ID_KEY)
        .and_then(|item| match &item.value {
            CborValue::Text(value) => Some(TopicId(value.clone())),
            _ => None,
        })
}

fn parse_task_id(invoke: &ActionInvoke) -> Result<TaskId, String> {
    let raw = invoke.argv.first().ok_or("usage: /factory start <id>")?;
    raw.parse::<u64>()
        .map(TaskId)
        .map_err(|_| format!("invalid task id `{raw}`"))
}

fn parse_topic_ref_and_rest(
    invoke: &ActionInvoke,
    usage: &'static str,
) -> Result<(String, String), String> {
    let topic_ref = invoke.argv.first().ok_or_else(|| usage.to_owned())?.clone();
    let rest = invoke.argv[1..].join(" ").trim().to_owned();
    if rest.is_empty() {
        return Err(usage.to_owned());
    }
    Ok((topic_ref, rest))
}

fn project_name_from_path(path: &Path) -> Result<String, String> {
    path.file_name()
        .and_then(|name| name.to_str())
        .filter(|name| !name.is_empty())
        .map(ToOwned::to_owned)
        .ok_or_else(|| format!("project path `{}` has no basename", path.display()))
}

fn factory_action_schema() -> ActionSchema {
    ActionSchema {
        version: tau_actions::ACTION_SCHEMA_VERSION,
        roots: vec![
            ActionCommand {
                name: "/factory".to_owned(),
                description: "Manage factory tasks".to_owned(),
                action_id: None,
                args: Vec::new(),
                children: vec![
                    ActionCommand {
                        name: "new".to_owned(),
                        description: "Create a new task from a title".to_owned(),
                        action_id: Some("new".to_owned()),
                        args: vec![ActionArg {
                            name: "title".to_owned(),
                            description: "Short task title".to_owned(),
                            required: true,
                            suggestions: Vec::new(),
                            kind: ActionArgKind::RestString,
                        }],
                        children: Vec::new(),
                    },
                    ActionCommand {
                        name: "start".to_owned(),
                        description: "Start an agent on a task in a fresh workspace".to_owned(),
                        action_id: Some("start".to_owned()),
                        args: vec![ActionArg {
                            name: "id".to_owned(),
                            description: "Task id, e.g. 1".to_owned(),
                            required: true,
                            suggestions: Vec::new(),
                            kind: ActionArgKind::Integer,
                        }],
                        children: Vec::new(),
                    },
                ],
            },
            ActionCommand {
                name: "/project".to_owned(),
                description: "Manage factory projects".to_owned(),
                action_id: None,
                args: Vec::new(),
                children: vec![ActionCommand {
                    name: "add".to_owned(),
                    description: "Add a project from a path".to_owned(),
                    action_id: Some("project_add".to_owned()),
                    args: vec![ActionArg {
                        name: "path".to_owned(),
                        description: "Project path; name is its basename".to_owned(),
                        required: true,
                        suggestions: Vec::new(),
                        kind: ActionArgKind::RestString,
                    }],
                    children: Vec::new(),
                }],
            },
            ActionCommand {
                name: "/topic".to_owned(),
                description: "Manage factory topics".to_owned(),
                action_id: None,
                args: Vec::new(),
                children: vec![
                    ActionCommand {
                        name: "new".to_owned(),
                        description: "Create and switch to a topic".to_owned(),
                        action_id: Some("topic_new".to_owned()),
                        args: vec![ActionArg {
                            name: "name".to_owned(),
                            description: "Topic name".to_owned(),
                            required: true,
                            suggestions: Vec::new(),
                            kind: ActionArgKind::RestString,
                        }],
                        children: Vec::new(),
                    },
                    ActionCommand {
                        name: "rename".to_owned(),
                        description: "Rename a topic".to_owned(),
                        action_id: Some("topic_rename".to_owned()),
                        args: vec![
                            ActionArg {
                                name: "topic".to_owned(),
                                description: "Topic id or name".to_owned(),
                                required: true,
                                suggestions: Vec::new(),
                                kind: ActionArgKind::String,
                            },
                            ActionArg {
                                name: "name".to_owned(),
                                description: "New topic name".to_owned(),
                                required: true,
                                suggestions: Vec::new(),
                                kind: ActionArgKind::RestString,
                            },
                        ],
                        children: Vec::new(),
                    },
                    ActionCommand {
                        name: "archive".to_owned(),
                        description: "Archive a topic".to_owned(),
                        action_id: Some("topic_archive".to_owned()),
                        args: vec![ActionArg {
                            name: "topic".to_owned(),
                            description: "Topic id or name".to_owned(),
                            required: true,
                            suggestions: Vec::new(),
                            kind: ActionArgKind::RestString,
                        }],
                        children: Vec::new(),
                    },
                ],
            },
        ],
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::task::Status;
    use std::io::Cursor;
    use std::path::Path;
    use tau_proto::{AgentId, HarnessInputReader, HarnessOutputWriter};

    fn init_jj_repo(repo_root: &Path) {
        smol::block_on(async {
            smol::fs::create_dir_all(repo_root)
                .await
                .expect("create repo dir");
            let status = smol::process::Command::new("jj")
                .arg("git")
                .arg("init")
                .arg(repo_root)
                .status()
                .await
                .expect("jj git init");
            assert!(status.success(), "jj git init failed");
            for (key, value) in [("user.name", "test"), ("user.email", "test@example.com")] {
                let output = smol::process::Command::new("jj")
                    .arg("--repository")
                    .arg(repo_root)
                    .arg("config")
                    .arg("set")
                    .arg("--repo")
                    .arg(key)
                    .arg(value)
                    .output()
                    .await
                    .expect("jj config set");
                assert!(output.status.success(), "jj config set {key} failed");
            }
        });
    }

    fn configure(state_dir: &Path, repo_root: &Path) -> HarnessOutputMessage {
        HarnessOutputMessage::Configure(Configure {
            instance_name: None,
            config: CborValue::Map(vec![(
                CborValue::Text("repo_root".to_owned()),
                CborValue::Text(repo_root.display().to_string()),
            )]),
            state_dir: Some(state_dir.to_path_buf()),
            debug_dir: None,
            secrets: std::collections::BTreeMap::new(),
        })
    }

    fn invoke(action_id: &str, args: &[&str]) -> HarnessOutputMessage {
        HarnessOutputMessage::deliver(Event::ActionInvoke(ActionInvoke {
            invocation_id: format!("inv-{action_id}").as_str().into(),
            extension_name: "tau-factory".into(),
            instance_id: 0.into(),
            action_id: action_id.to_owned(),
            raw_line: format!("/factory {action_id} {}", args.join(" ")),
            argv: args.iter().map(|arg| (*arg).to_owned()).collect(),
            arguments: CborValue::Map(Vec::new()),
        }))
    }

    fn agent_started(agent: &str, task_id: u64, cwd: &Path) -> HarnessOutputMessage {
        HarnessOutputMessage::deliver(Event::AgentStarted(AgentStarted {
            agent_id: AgentId::parse(agent).expect("agent id"),
            parent_agent: None,
            role: ROLE.to_owned(),
            display_name: None,
            metadata: vec![
                AgentInitialMetadata {
                    key: AgentMetadataKey::new(SHELL_CWD_KEY),
                    value: CborValue::Text(cwd.display().to_string()),
                    inheritable: true,
                },
                AgentInitialMetadata {
                    key: AgentMetadataKey::new(FACTORY_TASK_ID_KEY),
                    value: CborValue::Text(task_id.to_string()),
                    inheritable: true,
                },
            ],
        }))
    }

    fn drive(frames: &[HarnessOutputMessage]) -> Vec<HarnessInputMessage> {
        let mut input = Vec::new();
        let mut writer = HarnessOutputWriter::new(&mut input);
        for frame in frames {
            writer.write_message(frame).expect("write input frame");
        }
        writer.flush().expect("flush input");

        let mut output = Vec::new();
        run(Cursor::new(input), &mut output).expect("run");

        let mut reader = HarnessInputReader::new(Cursor::new(output));
        let mut messages = Vec::new();
        while let Some(message) = reader.read_message().expect("read") {
            messages.push(message);
        }
        messages
    }

    fn sync() -> HarnessOutputMessage {
        HarnessOutputMessage::deliver(Event::ExtensionEvent(
            CustomEvent::try_new(factory_event(SYNC_CALL), CborValue::Null)
                .expect("valid sync event"),
        ))
    }

    fn emitted(message: &HarnessInputMessage) -> Option<&Event> {
        match message {
            HarnessInputMessage::Emit(emit) => Some(emit.event.as_ref()),
            _ => None,
        }
    }

    fn tasks_updates(messages: &[HarnessInputMessage]) -> Vec<Vec<Task>> {
        messages
            .iter()
            .filter_map(|message| match emitted(message) {
                Some(Event::ExtensionEvent(custom))
                    if is_factory_event(custom.name(), TASKS_UPDATE_CALL) =>
                {
                    Some(
                        custom
                            .payload()
                            .deserialized::<Vec<Task>>()
                            .expect("tasks_update payload decodes to Vec<Task>"),
                    )
                }
                _ => None,
            })
            .collect()
    }

    fn projects_updates(messages: &[HarnessInputMessage]) -> Vec<Vec<Project>> {
        messages
            .iter()
            .filter_map(|message| match emitted(message) {
                Some(Event::ExtensionEvent(custom))
                    if is_factory_event(custom.name(), PROJECTS_UPDATE_CALL) =>
                {
                    Some(
                        custom
                            .payload()
                            .deserialized::<Vec<Project>>()
                            .expect("projects_update payload decodes to Vec<Project>"),
                    )
                }
                _ => None,
            })
            .collect()
    }

    fn topics_updates(messages: &[HarnessInputMessage]) -> Vec<Vec<Topic>> {
        messages
            .iter()
            .filter_map(|message| match emitted(message) {
                Some(Event::ExtensionEvent(custom))
                    if is_factory_event(custom.name(), TOPICS_UPDATE_CALL) =>
                {
                    Some(
                        custom
                            .payload()
                            .deserialized::<Vec<Topic>>()
                            .expect("topics_update payload decodes to Vec<Topic>"),
                    )
                }
                _ => None,
            })
            .collect()
    }

    #[test]
    fn start_creates_workspace_and_spawns_agent_bound_to_task() {
        let temp = tempfile::tempdir().expect("tempdir");
        let repo_root = temp.path().join("repo");
        let state_dir = temp.path().join("state");
        init_jj_repo(&repo_root);

        let workspace = state_dir.join("workspaces").join("task-1");

        let messages = drive(&[
            configure(&state_dir, &repo_root),
            invoke("new", &["Fix the parser crash"]),
            invoke("start", &["1"]),
            agent_started("agent-1", 1, &workspace),
        ]);

        // `start` emitted a UiCreateAgent for the invoking session with the
        // workspace as the shell cwd and the task binding stamped on.
        let create = messages
            .iter()
            .find_map(|message| match emitted(message) {
                Some(Event::UiCreateAgent(create)) => Some(create),
                _ => None,
            })
            .expect("start emits a UiCreateAgent");
        assert_eq!(create.role, ROLE);
        let cwd = create
            .metadata
            .iter()
            .find(|item| item.key.as_str() == SHELL_CWD_KEY)
            .expect("cwd metadata present");
        assert_eq!(cwd.value, CborValue::Text(workspace.display().to_string()));
        assert!(cwd.inheritable, "cwd must be inheritable for child agents");
        let bound = create
            .metadata
            .iter()
            .find(|item| item.key.as_str() == FACTORY_TASK_ID_KEY)
            .expect("task-id metadata present");
        assert_eq!(bound.value, CborValue::Text("1".to_owned()));

        // The jj workspace was actually created on disk.
        assert!(workspace.is_dir(), "workspace directory should exist");

        // After AgentStarted, the task is Active and bound to the agent.
        let store = TaskStore::open(&state_dir.join("factory.redb"));
        let task = store.get(TaskId(1)).expect("task exists");
        assert_eq!(
            task.status,
            Status::Active {
                worktree: workspace,
                agent: AgentId::parse("agent-1").unwrap(),
            }
        );
    }

    #[test]
    fn mutations_emit_deltas_and_sync_emits_the_whole_list() {
        let temp = tempfile::tempdir().expect("tempdir");
        let repo_root = temp.path().join("repo");
        let state_dir = temp.path().join("state");

        let messages = drive(&[
            configure(&state_dir, &repo_root),
            invoke("new", &["First task"]),
            invoke("new", &["Second task"]),
            sync(),
        ]);

        let titles = |update: &Vec<Task>| -> Vec<String> {
            update.iter().map(|task| task.title.clone()).collect()
        };

        // One update per `new` carrying only the task that changed, plus one
        // for the explicit sync request carrying the whole board (id order).
        let updates = tasks_updates(&messages);
        assert_eq!(updates.len(), 3, "two deltas and one sync snapshot");
        assert_eq!(titles(&updates[0]), vec!["First task".to_owned()]);
        assert_eq!(titles(&updates[1]), vec!["Second task".to_owned()]);
        assert_eq!(
            titles(&updates[2]),
            vec!["First task".to_owned(), "Second task".to_owned()]
        );
    }

    #[test]
    fn project_add_derives_name_from_path_and_publishes_full_project_list() {
        let temp = tempfile::tempdir().expect("tempdir");
        let repo_root = temp.path().join("repo");
        let state_dir = temp.path().join("state");
        let first = temp.path().join("alpha");
        let second = temp.path().join("nested").join("beta");

        let messages = drive(&[
            configure(&state_dir, &repo_root),
            invoke("project_add", &[first.to_str().unwrap()]),
            invoke("project_add", &[second.to_str().unwrap()]),
            sync(),
        ]);

        let updates = projects_updates(&messages);
        assert_eq!(
            updates.len(),
            3,
            "two full-list mutations and one sync snapshot"
        );
        assert_eq!(
            updates[0]
                .iter()
                .map(|project| &project.name)
                .collect::<Vec<_>>(),
            vec!["alpha"]
        );
        assert_eq!(
            updates[1]
                .iter()
                .map(|project| &project.name)
                .collect::<Vec<_>>(),
            vec!["alpha", "beta"]
        );
        assert_eq!(
            updates[2]
                .iter()
                .map(|project| &project.name)
                .collect::<Vec<_>>(),
            vec!["alpha", "beta"]
        );
        assert_eq!(updates[1][1].path, second);
    }

    #[test]
    fn topic_new_emits_topic_delta_and_sync_emits_all_topics() {
        let temp = tempfile::tempdir().expect("tempdir");
        let repo_root = temp.path().join("repo");
        let state_dir = temp.path().join("state");

        let messages = drive(&[
            configure(&state_dir, &repo_root),
            invoke("topic_new", &["Sidebar redesign"]),
            sync(),
        ]);

        let updates = topics_updates(&messages);
        assert_eq!(updates.len(), 2, "topic delta and sync snapshot");
        assert_eq!(updates[0].len(), 1);
        assert_eq!(updates[0][0].name, "Sidebar redesign");
        assert!(updates[0][0].id.0.starts_with("tp-"));
        assert_eq!(updates[0][0].id.0.len(), 9);
        assert!(
            updates[1]
                .iter()
                .any(|topic| topic.name == "(no topic)" && topic.id.0 == "tp-000000")
        );
        assert!(
            updates[1]
                .iter()
                .any(|topic| topic.name == "Sidebar redesign")
        );
    }

    #[test]
    fn start_on_unknown_task_errors_without_spawning() {
        let temp = tempfile::tempdir().expect("tempdir");
        let repo_root = temp.path().join("repo");
        let state_dir = temp.path().join("state");
        init_jj_repo(&repo_root);

        let messages = drive(&[configure(&state_dir, &repo_root), invoke("start", &["99"])]);

        let error = messages
            .iter()
            .find_map(|message| match emitted(message) {
                Some(Event::ActionError(error)) => Some(error),
                _ => None,
            })
            .expect("start on a missing task returns an action error");
        assert!(error.message.contains("no such task"));
        assert!(
            messages
                .iter()
                .all(|message| !matches!(emitted(message), Some(Event::UiCreateAgent(_)))),
            "no agent should be requested for a missing task"
        );
    }

    /// Smoke test against the real prebuilt `tau` daemon: enables the factory
    /// via a self-removing config drop-in, launches `tau ext harness`, and
    /// confirms from the daemon log that the harness spawned the factory binary
    /// and that the factory completed its handshake — the harness only reaches
    /// "extensions ready" once every extension, the factory included, sends its
    /// `Ready`. Invoking the `/factory` actions live needs a full tau UI client
    /// (connection + follower lifecycle); the protocol-level unit tests above
    /// already cover the factory's action and `AgentStarted` handling.
    ///
    /// Ignored by default (spawns the daemon and touches `~/.config/tau`); run:
    ///   cargo test -p tau_factory live_factory_loads -- --ignored --nocapture
    #[test]
    #[ignore = "live: spawns the real tau daemon"]
    #[allow(clippy::disallowed_methods)]
    fn live_factory_loads_in_real_harness() {
        use std::io::BufWriter;
        use std::process::{Command, Stdio};
        use std::time::{Duration, Instant};
        use tau_proto::{
            ClientKind, EventSelector, ExtensionName, Hello, PROTOCOL_VERSION, Subscribe,
        };

        let tau_bin = "/home/maan2003/src/tau/target/debug/tau";
        let factory_bin = concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/../../target/debug/tau-factory"
        );
        assert!(Path::new(tau_bin).exists(), "prebuilt tau binary missing");
        assert!(
            Path::new(factory_bin).exists(),
            "build the factory bin first"
        );

        let temp = tempfile::tempdir().expect("tempdir");
        let repo = temp.path().join("repo");
        init_jj_repo(&repo);

        // Enable the factory via an additive, self-removing config drop-in.
        let home = std::env::var("HOME").expect("HOME");
        let dropin_dir = PathBuf::from(&home).join(".config/tau/harness.d");
        let created_dir = !dropin_dir.exists();
        std::fs::create_dir_all(&dropin_dir).expect("create drop-in dir");
        let dropin = dropin_dir.join("zz-factory-live.yaml");
        std::fs::write(
            &dropin,
            format!(
                "extensions:\n  factory:\n    command: [\"{factory_bin}\"]\n    enable: true\n"
            ),
        )
        .expect("write drop-in");
        struct Cleanup {
            file: PathBuf,
            dir: Option<PathBuf>,
        }
        impl Drop for Cleanup {
            fn drop(&mut self) {
                let _ = std::fs::remove_file(&self.file);
                if let Some(dir) = &self.dir {
                    let _ = std::fs::remove_dir(dir);
                }
            }
        }
        let _cleanup = Cleanup {
            file: dropin,
            dir: created_dir.then_some(dropin_dir),
        };

        let log = temp.path().join("daemon.log");
        let mut child = Command::new(tau_bin)
            .arg("ext")
            .arg("harness")
            .current_dir(&repo)
            .env("TAU_SESSION_ID", "factory-live")
            .env("TAU_SESSION_STATUS", "new")
            .env("TAU_LOG", "tau_harness=debug")
            .stdin(Stdio::piped())
            .stdout(Stdio::piped())
            .stderr(Stdio::from(std::fs::File::create(&log).expect("log file")))
            .spawn()
            .expect("spawn tau ext harness");

        // The harness blocks at startup until the initial UI subscribes, so send
        // a minimal Hello + Subscribe to let it proceed to spawning extensions.
        // Hold `writer` (the daemon's stdin) open for the duration so the daemon
        // does not see the UI disconnect mid-startup.
        let stdin = child.stdin.take().expect("daemon stdin");
        let mut writer = PeerOutputWriter::new(BufWriter::new(stdin));
        writer
            .write_message(&HarnessInputMessage::Hello(Hello {
                protocol_version: PROTOCOL_VERSION,
                client_name: ExtensionName::from("factory-live"),
                client_kind: ClientKind::Ui,
            }))
            .expect("hello");
        writer
            .write_message(&HarnessInputMessage::Subscribe(Subscribe {
                selectors: vec![EventSelector::Prefix("harness.".to_owned())],
            }))
            .expect("subscribe");
        writer.flush().expect("flush handshake");

        // Poll the daemon log until startup completes (or time out).
        let deadline = Instant::now() + Duration::from_secs(20);
        let mut log_text = String::new();
        while Instant::now() < deadline {
            log_text = std::fs::read_to_string(&log).unwrap_or_default();
            if log_text.contains("daemon ready markers written") {
                break;
            }
            std::thread::sleep(Duration::from_millis(100));
        }
        let _ = child.kill();
        let _ = child.wait();

        eprintln!("---- daemon.log ----\n{log_text}\n---- end daemon.log ----");
        assert!(
            log_text.contains("extension spawned extension=factory"),
            "the real harness should launch the factory binary"
        );
        assert!(
            log_text.contains("extensions ready"),
            "the factory should complete its handshake so the harness reaches 'extensions ready'"
        );
    }
}
