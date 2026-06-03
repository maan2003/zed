use std::collections::HashMap;

use crate::transcript::InsertedTranscript;

#[derive(Default)]
pub(crate) struct ToolState {
    pending_calls: HashMap<String, InsertedTranscript>,
    delegate_progress: HashMap<String, tau_proto::DelegateProgress>,
}

impl ToolState {
    pub(crate) fn contains_pending(&self, call_id: &str) -> bool {
        self.pending_calls.contains_key(call_id)
    }

    pub(crate) fn take_pending(&mut self, call_id: &str) -> Option<InsertedTranscript> {
        self.pending_calls.remove(call_id)
    }

    pub(crate) fn insert_pending(&mut self, call_id: String, inserted: InsertedTranscript) {
        self.pending_calls.insert(call_id, inserted);
    }

    pub(crate) fn record_delegate_progress(&mut self, progress: &tau_proto::DelegateProgress) {
        self.delegate_progress
            .insert(progress.call_id.to_string(), progress.clone());
    }

    pub(crate) fn finish_call(&mut self, call_id: &str) {
        self.delegate_progress.remove(call_id);
    }

    pub(crate) fn live_delegate_tools_status_chip(&self) -> Option<String> {
        self.delegate_progress
            .values()
            .find_map(delegate_progress_tools_status_chip)
    }
}

fn delegate_progress_tools_status_chip(progress: &tau_proto::DelegateProgress) -> Option<String> {
    progress
        .display
        .as_ref()
        .and_then(|display| {
            display
                .progress_counters
                .iter()
                .find_map(tools_progress_counter_status_chip)
        })
        .or_else(|| {
            (progress.tools_total != 0).then(|| {
                format!(
                    "{}/{}",
                    progress
                        .tools_total
                        .saturating_sub(progress.tools_in_flight),
                    progress.tools_total
                )
            })
        })
}

fn tools_progress_counter_status_chip(counter: &tau_proto::ProgressCounter) -> Option<String> {
    if counter.label.as_deref() != Some("tools") || counter.unit != tau_proto::ProgressUnit::Count {
        return None;
    }
    Some(match (counter.complete, counter.total) {
        (Some(complete), Some(total)) => format!("{complete}/{total}"),
        (Some(complete), None) => complete.to_string(),
        (None, Some(total)) => format!("-/{total}"),
        (None, None) => "-".to_owned(),
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn default_state_has_no_pending_calls() {
        let state = ToolState::default();

        assert!(!state.contains_pending("call"));
    }

    #[test]
    fn delegate_progress_uses_tools_progress_counter() {
        let mut state = ToolState::default();
        let progress = tau_proto::DelegateProgress {
            call_id: "call".into(),
            task_name: "task".to_owned(),
            agent_id: None,
            role: None,
            ctx_percent: None,
            ctx_input_tokens: None,
            ctx_window: None,
            tools_in_flight: 9,
            tools_total: 10,
            display: Some(tau_proto::ToolUseState {
                progress_counters: vec![tau_proto::ProgressCounter {
                    label: Some("tools".to_owned()),
                    unit: tau_proto::ProgressUnit::Count,
                    complete: Some(3),
                    total: Some(7),
                }],
                ..Default::default()
            }),
        };

        state.record_delegate_progress(&progress);

        assert_eq!(
            state.live_delegate_tools_status_chip().as_deref(),
            Some("3/7")
        );
    }

    #[test]
    fn delegate_progress_falls_back_to_total_and_in_flight() {
        let mut state = ToolState::default();
        let progress = tau_proto::DelegateProgress {
            call_id: "call".into(),
            task_name: "task".to_owned(),
            agent_id: None,
            role: None,
            ctx_percent: None,
            ctx_input_tokens: None,
            ctx_window: None,
            tools_in_flight: 2,
            tools_total: 5,
            display: None,
        };

        state.record_delegate_progress(&progress);
        assert_eq!(
            state.live_delegate_tools_status_chip().as_deref(),
            Some("3/5")
        );

        state.finish_call("call");
        assert_eq!(state.live_delegate_tools_status_chip(), None);
    }
}
