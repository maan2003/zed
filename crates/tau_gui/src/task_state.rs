//! The board's projection of the factory's tasks.
//!
//! The factory owns the durable task store; the UI keeps a derived, in-memory
//! mirror folded from the factory's `factory.tasks_update` custom events. Each
//! event carries a `Vec<Task>` that is a batch of upserts keyed by `TaskId`:
//! just the changed task on a mutation, the whole list in response to the
//! `factory.sync` the UI emits on connect. Tasks are never removed (closing is a
//! status), so the fold is a pure upsert with no tombstones.
//!
//! Ordering is a render concern, not a wire concern: the factory imposes none,
//! so the board groups by attention first (what needs the human) and then by
//! status here.

use std::collections::BTreeMap;

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

    pub(crate) fn len(&self) -> usize {
        self.tasks.len()
    }

    /// How many tasks are waiting on the human right now — the number the board
    /// exists to keep visible.
    pub(crate) fn attention_count(&self) -> usize {
        self.tasks
            .values()
            .filter(|task| task.attention.is_some())
            .count()
    }

    /// The board as text: an attention-first, then status-grouped listing.
    pub(crate) fn render_board(&self) -> String {
        if self.tasks.is_empty() {
            return "Tasks\n\n  No tasks yet. Create one with /factory new <title>.\n".to_owned();
        }

        let mut out = format!(
            "Tasks — {} of {} need you\n",
            self.attention_count(),
            self.len()
        );

        // Anything waiting on the human, regardless of status, most urgent
        // first. This is the board's reason to exist, so it leads.
        let mut needs_you: Vec<&Task> = self
            .tasks
            .values()
            .filter(|task| task.attention.is_some())
            .collect();
        needs_you.sort_by_key(|task| (attention_priority(task.attention), task.id));
        push_section(
            &mut out,
            "Needs you",
            needs_you.iter().map(|task| task_line(task, true)).collect(),
        );

        // The rest, grouped by where they are in their lifecycle. A task only
        // appears here when it is not already under "Needs you".
        for (heading, matches) in STATUS_SECTIONS {
            let lines = self
                .tasks
                .values()
                .filter(|task| task.attention.is_none() && matches(&task.status))
                .map(|task| task_line(task, false))
                .collect();
            push_section(&mut out, heading, lines);
        }
        out
    }
}

/// The `HarnessInputMessage` the UI emits to ask the factory for the current
/// board. Custom events are not replayed, so this request is how a freshly
/// connected UI learns the tasks that already exist.
pub(crate) fn sync_request() -> HarnessInputMessage {
    let name = EventName::new(EventCategory::Other(wire::CATEGORY.to_owned()), wire::SYNC);
    let event = CustomEvent::try_new(name, None, CborValue::Null)
        .expect("factory.sync is a valid extension-owned event name");
    HarnessInputMessage::emit(Event::ExtensionEvent(event))
}

const STATUS_SECTIONS: [(&str, fn(&Status) -> bool); 4] = [
    ("Active", |status| matches!(status, Status::Active { .. })),
    ("Open", |status| matches!(status, Status::Open)),
    ("Done", |status| matches!(status, Status::Done { .. })),
    ("Closed", |status| matches!(status, Status::Closed { .. })),
];

fn is_tasks_update(name: &EventName) -> bool {
    name.category().as_str() == wire::CATEGORY && name.call().as_str() == wire::TASKS_UPDATE
}

fn push_section(out: &mut String, heading: &str, lines: Vec<String>) {
    if lines.is_empty() {
        return;
    }
    out.push_str(&format!("\n  {heading}\n"));
    for line in lines {
        out.push_str(&line);
    }
}

fn task_line(task: &Task, show_attention: bool) -> String {
    let marker = if show_attention {
        attention_label(task.attention)
    } else {
        ""
    };
    let agent = status_agent(&task.status).unwrap_or_default();
    let line = format!(
        "    {id:<8}  {marker:<9}  {title:<46}  {agent}",
        id = task.id.to_string(),
        title = truncate(&task.title, 46),
    );
    let mut line = line.trim_end().to_owned();
    line.push('\n');
    line
}

fn attention_priority(attention: Option<Attention>) -> u8 {
    match attention {
        Some(Attention::Decision) => 0,
        Some(Attention::Question) => 1,
        Some(Attention::Review) => 2,
        None => 3,
    }
}

fn attention_label(attention: Option<Attention>) -> &'static str {
    match attention {
        Some(Attention::Decision) => "decision",
        Some(Attention::Question) => "question",
        Some(Attention::Review) => "review",
        None => "",
    }
}

fn status_agent(status: &Status) -> Option<String> {
    match status {
        Status::Open => None,
        Status::Active { agent, .. } | Status::Done { agent } => Some(agent.to_string()),
        Status::Closed { agent } => agent.as_ref().map(ToString::to_string),
    }
}

/// Truncate to at most `max` characters (not bytes, so multi-byte titles never
/// panic), marking elision with `…`.
fn truncate(text: &str, max: usize) -> String {
    if text.chars().count() <= max {
        return text.to_owned();
    }
    let mut truncated: String = text.chars().take(max.saturating_sub(1)).collect();
    truncated.push('…');
    truncated
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
        state.observe_event(&tasks_update(&[task(1, "First renamed", Status::Open, None)]));
        assert_eq!(state.len(), 2);
        assert!(state.render_board().contains("First renamed"));
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
        let board = state.render_board();
        let needs_you = board.find("Needs you").expect("needs-you section");
        // The plain open task is the only one without attention, so it forms the
        // single status section; the active+review task is pulled up into
        // "Needs you" instead of appearing under an "Active" section.
        let open = board.find("\n  Open").expect("open section");
        assert!(needs_you < open, "needs-you section must come first");
        assert!(
            !board.contains("\n  Active"),
            "the active task has attention, so it belongs under Needs you, not Active"
        );
        // The active+review task shows under Needs you with its attention label.
        assert!(board.contains("review"));
        assert!(board.contains("Patch ready"));
        assert!(board.contains("impl-2"));
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
