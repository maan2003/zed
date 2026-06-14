//! The task model and its legal transitions.
//!
//! A task is an issue plus the patch an agent produces for it. The model has two
//! independent axes: `Status` (where the task is in its lifecycle, carrying the
//! data that only exists in that state) and `Attention` (what, if anything, is
//! waiting on the human — orthogonal to status). The transition methods are the
//! only way to move a task between statuses; each validates the current status
//! so an illegal move (e.g. completing a task no agent ever started) is a typed
//! error rather than a silently corrupt state.

use std::path::PathBuf;

use serde::{Deserialize, Serialize};
use tau_proto::AgentId;

/// When something happened. RFC 3339 in the durable store; UTC in memory.
pub type Timestamp = chrono::DateTime<chrono::Utc>;

/// The custom-event names the factory extension and the board UI exchange.
///
/// These are the wire contract between the two halves: the UI emits
/// `factory.sync` to ask for the current board, and the factory emits
/// `factory.tasks_update` carrying a `Vec<Task>` (a batch of upserts — just the
/// changed task on a mutation, the whole list in response to a sync). Both
/// sides reference these so the names cannot drift apart.
pub mod wire {
    /// Event category owning the factory's custom events. Not a reserved tau
    /// category, so the harness routes it as extension-owned.
    pub const CATEGORY: &str = "factory";
    /// Call segment of `factory.sync` — the UI's request for the task list.
    pub const SYNC: &str = "sync";
    /// Call segment of `factory.tasks_update` — the factory's `Vec<Task>` payload.
    pub const TASKS_UPDATE: &str = "tasks_update";
}

/// Stable, human-legible task identifier. Allocated by the store as a monotonic
/// counter so a small fleet reads as `task-1`, `task-2`, … rather than opaque
/// hashes.
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub struct TaskId(pub u64);

impl std::fmt::Display for TaskId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "task-{}", self.0)
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct Task {
    pub id: TaskId,
    pub title: String,
    /// The problem statement, in markdown.
    pub issue: String,
    pub status: Status,
    /// What is waiting on the human, in any status. `None` means the task needs
    /// nothing from you right now.
    pub attention: Option<Attention>,
    pub created_at: Timestamp,
    pub updated_at: Timestamp,
}

/// Where a task is in its lifecycle. Each variant carries exactly the data that
/// only exists in that state.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum Status {
    /// Backlog: the issue exists, but no workspace and no agent yet.
    Open,
    /// An agent is working it — a workspace and the agent bound to it both exist.
    Active { worktree: PathBuf, agent: AgentId },
    /// Merged. It necessarily had an agent.
    Done { agent: AgentId },
    /// Abandoned. `None` when closed straight from `Open`, before any agent ran.
    Closed { agent: Option<AgentId> },
}

impl Status {
    /// Lower-case label for error messages.
    fn label(&self) -> &'static str {
        match self {
            Status::Open => "open",
            Status::Active { .. } => "active",
            Status::Done { .. } => "done",
            Status::Closed { .. } => "closed",
        }
    }
}

/// What a task is waiting on from the human — the orthogonal axis the board
/// groups by.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum Attention {
    /// A choice only the human can make.
    Decision,
    /// The agent asked something and is blocked on the answer.
    Question,
    /// A patch is ready to look at.
    Review,
}

/// A transition was attempted from a status that does not allow it.
#[derive(Debug, thiserror::Error, PartialEq, Eq)]
#[error("cannot {action} a task that is {from}")]
pub struct InvalidTransition {
    pub action: &'static str,
    pub from: &'static str,
}

impl Task {
    /// `Open` → `Active`: bind the task to a freshly created workspace and the
    /// agent now running in it.
    pub fn start(&mut self, worktree: PathBuf, agent: AgentId) -> Result<(), InvalidTransition> {
        match self.status {
            Status::Open => {
                self.status = Status::Active { worktree, agent };
                Ok(())
            }
            ref other => Err(InvalidTransition {
                action: "start",
                from: other.label(),
            }),
        }
    }

    /// `Active` → `Done`: the patch merged. Carries the agent forward.
    pub fn complete(&mut self) -> Result<(), InvalidTransition> {
        match &self.status {
            Status::Active { agent, .. } => {
                let agent = agent.clone();
                self.status = Status::Done { agent };
                Ok(())
            }
            other => Err(InvalidTransition {
                action: "complete",
                from: other.label(),
            }),
        }
    }

    /// `Open`/`Active` → `Closed`: abandon the task, remembering the agent that
    /// worked it if there was one.
    pub fn close(&mut self) -> Result<(), InvalidTransition> {
        match &self.status {
            Status::Open => {
                self.status = Status::Closed { agent: None };
                Ok(())
            }
            Status::Active { agent, .. } => {
                let agent = agent.clone();
                self.status = Status::Closed { agent: Some(agent) };
                Ok(())
            }
            other => Err(InvalidTransition {
                action: "close",
                from: other.label(),
            }),
        }
    }

    /// Raise or change the human-attention flag. Valid in any status, since
    /// attention is orthogonal to the lifecycle.
    pub fn flag(&mut self, attention: Attention) {
        self.attention = Some(attention);
    }

    /// Clear the human-attention flag.
    pub fn unflag(&mut self) {
        self.attention = None;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn agent(name: &str) -> AgentId {
        AgentId::parse(name).expect("valid agent id")
    }

    fn open_task() -> Task {
        let now = chrono::Utc::now();
        Task {
            id: TaskId(1),
            title: "Fix the parser crash".into(),
            issue: "stack overflow on deeply nested input".into(),
            status: Status::Open,
            attention: None,
            created_at: now,
            updated_at: now,
        }
    }

    #[test]
    fn start_binds_workspace_and_agent() {
        let mut task = open_task();
        task.start(PathBuf::from("/tmp/ws"), agent("impl-1")).unwrap();
        assert_eq!(
            task.status,
            Status::Active {
                worktree: PathBuf::from("/tmp/ws"),
                agent: agent("impl-1"),
            }
        );
    }

    #[test]
    fn complete_carries_the_agent_forward() {
        let mut task = open_task();
        task.start(PathBuf::from("/tmp/ws"), agent("impl-1")).unwrap();
        task.complete().unwrap();
        assert_eq!(task.status, Status::Done { agent: agent("impl-1") });
    }

    #[test]
    fn close_from_open_has_no_agent() {
        let mut task = open_task();
        task.close().unwrap();
        assert_eq!(task.status, Status::Closed { agent: None });
    }

    #[test]
    fn close_from_active_remembers_the_agent() {
        let mut task = open_task();
        task.start(PathBuf::from("/tmp/ws"), agent("impl-1")).unwrap();
        task.close().unwrap();
        assert_eq!(
            task.status,
            Status::Closed {
                agent: Some(agent("impl-1")),
            }
        );
    }

    #[test]
    fn completing_an_open_task_is_rejected() {
        let mut task = open_task();
        let error = task.complete().unwrap_err();
        assert_eq!(
            error,
            InvalidTransition {
                action: "complete",
                from: "open",
            }
        );
        // The rejected transition left the task untouched.
        assert_eq!(task.status, Status::Open);
    }

    #[test]
    fn attention_is_independent_of_status() {
        let mut task = open_task();
        task.flag(Attention::Review);
        task.start(PathBuf::from("/tmp/ws"), agent("impl-1")).unwrap();
        // The status moved, the flag did not.
        assert_eq!(task.attention, Some(Attention::Review));
        task.unflag();
        assert_eq!(task.attention, None);
    }
}
