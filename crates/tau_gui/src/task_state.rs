//! The board's projection of the factory's tasks.
//!
//! The factory owns the durable task store; the UI keeps a derived, in-memory
//! mirror folded from the factory's `factory.tasks_update` custom events. Each
//! event carries a `Vec<Task>` that is a batch of upserts keyed by `TaskId`:
//! just the changed task on a mutation, the whole list in response to the
//! `factory.sync` the UI emits on connect. Tasks are never removed (closing is a
//! status), so the fold is a pure upsert with no tombstones.

use std::collections::BTreeMap;
use std::ops::Range;

use tau_proto::{CborValue, CustomEvent, Event, EventCategory, EventName, HarnessInputMessage};
use tau_task::{Attention, Status, Task, TaskId, wire};

#[derive(Default)]
pub(crate) struct TaskState {
    tasks: BTreeMap<TaskId, Task>,
}

impl TaskState {
    /// Folds a `factory.tasks_update` into the mirror. Any other event is
    /// ignored, so this can be called for every delivered event.
    pub(crate) fn observe_event(&mut self, event: &Event) {
        let Event::ExtensionEvent(custom) = event else {
            return;
        };
        if !is_tasks_update(custom.name()) {
            return;
        }
        match custom.payload().deserialized::<Vec<Task>>() {
            Ok(tasks) => {
                for task in tasks {
                    self.tasks.insert(task.id, task);
                }
            }
            Err(error) => {
                eprintln!("tau-gui: ignoring malformed factory.tasks_update: {error}");
            }
        }
    }

    #[cfg(test)]
    pub(crate) fn len(&self) -> usize {
        self.tasks.len()
    }

    /// How many tasks are waiting on the human right now — useful internally and
    /// in tests, even though the board intentionally does not print the count.
    #[cfg(test)]
    pub(crate) fn attention_count(&self) -> usize {
        self.tasks
            .values()
            .filter(|task| task.attention.is_some())
            .count()
    }

    pub(crate) fn task_agent(&self, id: TaskId) -> Option<String> {
        self.tasks.get(&id).and_then(|task| match &task.status {
            Status::Active { agent, .. } | Status::Done { agent } => Some(agent.to_string()),
            Status::Closed { agent } => agent.as_ref().map(ToString::to_string),
            Status::Open => None,
        })
    }

    pub(crate) fn mini_rows(&self) -> Vec<TaskMiniRow> {
        self.ordered_rows()
            .into_iter()
            .filter(|task| !matches!(task.status, Status::Done { .. } | Status::Closed { .. }))
            .take(12)
            .map(TaskMiniRow::from)
            .collect()
    }

    /// The full task board as generated text, plus task anchor ranges expressed
    /// as byte offsets in that text. The UI turns these offsets into buffer
    /// anchors after inserting the text.
    pub(crate) fn render_full_board(&self) -> BoardRender {
        if self.tasks.is_empty() {
            return BoardRender {
                text: "Tasks\n\n  No tasks yet. Create one with /factory new <title>.\n".to_owned(),
                rows: Vec::new(),
            };
        }

        let mut render = BoardRender {
            text: "Tasks\n".to_owned(),
            rows: Vec::new(),
        };

        self.push_section(
            &mut render,
            "Needs you",
            self.tasks
                .values()
                .filter(|task| task.attention.is_some())
                .collect(),
            None,
        );
        self.push_section(
            &mut render,
            "Active",
            self.tasks
                .values()
                .filter(|task| {
                    task.attention.is_none() && matches!(task.status, Status::Active { .. })
                })
                .collect(),
            None,
        );
        self.push_section(
            &mut render,
            "Open",
            self.tasks
                .values()
                .filter(|task| task.attention.is_none() && matches!(task.status, Status::Open))
                .collect(),
            Some(3),
        );

        if self
            .tasks
            .values()
            .any(|task| task.attention.is_none() && matches!(task.status, Status::Done { .. }))
        {
            render.text.push_str("\n[+] Done\n");
        }
        if self
            .tasks
            .values()
            .any(|task| task.attention.is_none() && matches!(task.status, Status::Closed { .. }))
        {
            render.text.push_str("\n[+] Closed\n");
        }

        render
    }

    fn ordered_rows(&self) -> Vec<&Task> {
        let mut tasks = self.tasks.values().collect::<Vec<_>>();
        tasks.sort_by_key(|task| {
            (
                section_priority(task),
                attention_priority(task.attention),
                std::cmp::Reverse(task.updated_at),
                task.id,
            )
        });
        tasks
    }

    fn push_section(
        &self,
        render: &mut BoardRender,
        heading: &str,
        mut tasks: Vec<&Task>,
        limit: Option<usize>,
    ) {
        if tasks.is_empty() {
            return;
        }
        tasks.sort_by_key(|task| {
            (
                attention_priority(task.attention),
                std::cmp::Reverse(task.updated_at),
                task.id,
            )
        });

        render.text.push_str(&format!("\n{heading}\n"));
        let shown = limit.unwrap_or(tasks.len()).min(tasks.len());
        for task in tasks.iter().take(shown) {
            let start = render.text.len();
            render.text.push_str("  ");
            render.text.push_str(&task.title);
            render.text.push('\n');
            let end = render.text.len();
            render.rows.push(BoardRowRange {
                task_id: task.id,
                range: start..end,
            });
        }
        if shown < tasks.len() {
            render.text.push_str("  … more\n");
        }
    }
}

pub(crate) struct BoardRender {
    pub(crate) text: String,
    pub(crate) rows: Vec<BoardRowRange>,
}

pub(crate) struct BoardRowRange {
    pub(crate) task_id: TaskId,
    pub(crate) range: Range<usize>,
}

#[derive(Clone)]
pub(crate) struct TaskMiniRow {
    pub(crate) id: TaskId,
    pub(crate) title: String,
    pub(crate) kind: TaskVisualKind,
}

impl From<&Task> for TaskMiniRow {
    fn from(task: &Task) -> Self {
        Self {
            id: task.id,
            title: task.title.clone(),
            kind: TaskVisualKind::from_task(task),
        }
    }
}

#[derive(Clone, Copy, PartialEq, Eq)]
pub(crate) enum TaskVisualKind {
    Decision,
    Question,
    Review,
    Active,
    Open,
}

impl TaskVisualKind {
    fn from_task(task: &Task) -> Self {
        match task.attention {
            Some(Attention::Decision) => Self::Decision,
            Some(Attention::Question) => Self::Question,
            Some(Attention::Review) => Self::Review,
            None => match task.status {
                Status::Active { .. } => Self::Active,
                Status::Open => Self::Open,
                Status::Done { .. } | Status::Closed { .. } => Self::Open,
            },
        }
    }
}

/// The `HarnessInputMessage` the UI emits to ask the factory for the current
/// board. Custom events are not replayed, so this request is how a freshly
/// connected UI learns the tasks that already exist.
pub(crate) fn sync_request() -> HarnessInputMessage {
    let name = EventName::new(EventCategory::Other(wire::CATEGORY.to_owned()), wire::SYNC);
    let event = CustomEvent::try_new(name, CborValue::Null)
        .expect("factory.sync is a valid extension-owned event name");
    HarnessInputMessage::emit(Event::ExtensionEvent(event))
}

fn is_tasks_update(name: &EventName) -> bool {
    name.category().as_str() == wire::CATEGORY && name.call().as_str() == wire::TASKS_UPDATE
}

fn attention_priority(attention: Option<Attention>) -> u8 {
    match attention {
        Some(Attention::Decision) => 0,
        Some(Attention::Question) => 1,
        Some(Attention::Review) => 2,
        None => 3,
    }
}

fn section_priority(task: &Task) -> u8 {
    if task.attention.is_some() {
        return 0;
    }
    match task.status {
        Status::Active { .. } => 1,
        Status::Open => 2,
        Status::Done { .. } => 3,
        Status::Closed { .. } => 4,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::PathBuf;

    fn agent(name: &str) -> tau_proto::AgentId {
        tau_proto::AgentId::parse(name).expect("valid agent id")
    }

    fn task(id: u64, title: &str, status: Status, attention: Option<Attention>) -> Task {
        let now = chrono::Utc::now();
        Task {
            id: TaskId(id),
            title: title.to_owned(),
            issue: String::new(),
            status,
            attention,
            created_at: now,
            updated_at: now,
        }
    }

    fn tasks_update(tasks: &[Task]) -> Event {
        let name = EventName::new(
            EventCategory::Other(wire::CATEGORY.to_owned()),
            wire::TASKS_UPDATE,
        );
        let payload = CborValue::serialized(&tasks.to_vec()).expect("serialize tasks");
        Event::ExtensionEvent(CustomEvent::try_new(name, None, payload).expect("valid event"))
    }

    #[test]
    fn folds_updates_as_upserts_keyed_by_id() {
        let mut state = TaskState::default();
        state.observe_event(&tasks_update(&[
            task(1, "First", Status::Open, None),
            task(2, "Second", Status::Open, None),
        ]));
        assert_eq!(state.len(), 2);

        // A later delta for an existing id replaces, not appends.
        state.observe_event(&tasks_update(&[task(
            1,
            "First renamed",
            Status::Open,
            None,
        )]));
        assert_eq!(state.len(), 2);
        assert!(state.render_full_board().text.contains("First renamed"));
    }

    #[test]
    fn attention_tasks_lead_regardless_of_status() {
        let mut state = TaskState::default();
        state.observe_event(&tasks_update(&[
            task(1, "Plain open", Status::Open, None),
            task(
                2,
                "Patch ready",
                Status::Active {
                    worktree: PathBuf::from("/tmp/ws"),
                    agent: agent("impl-2"),
                },
                Some(Attention::Review),
            ),
        ]));

        assert_eq!(state.attention_count(), 1);
        let board = state.render_full_board().text;
        let needs_you = board.find("Needs you").expect("needs-you section");
        let open = board.find("\nOpen").expect("open section");
        assert!(needs_you < open, "needs-you section must come first");
        assert!(
            !board.contains("\nActive"),
            "the active task has attention, so it belongs under Needs you, not Active"
        );
        assert!(board.contains("Patch ready"));
        assert!(!board.contains("impl-2"));
    }

    #[test]
    fn open_tasks_are_limited_by_default() {
        let mut state = TaskState::default();
        state.observe_event(&tasks_update(&[
            task(1, "One", Status::Open, None),
            task(2, "Two", Status::Open, None),
            task(3, "Three", Status::Open, None),
            task(4, "Four", Status::Open, None),
        ]));

        let board = state.render_full_board().text;
        assert!(board.contains("… more"));
        assert_eq!(
            board.matches("\n  ").count(),
            4,
            "three tasks plus more row"
        );
    }

    #[test]
    fn non_factory_events_are_ignored() {
        let mut state = TaskState::default();
        state.observe_event(&Event::ExtensionEvent(
            CustomEvent::try_new(
                EventName::new(EventCategory::Other("other".to_owned()), "thing"),
                None,
                CborValue::Null,
            )
            .expect("valid event"),
        ));
        assert_eq!(state.len(), 0);
    }
}
