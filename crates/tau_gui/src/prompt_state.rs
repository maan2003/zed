use std::collections::HashMap;

use crate::transcript::InsertedTranscript;

#[derive(Default)]
pub(crate) struct PromptState {
    streamed_responses: HashMap<String, String>,
    live_response_ranges: HashMap<String, InsertedTranscript>,
    live_compaction_ranges: HashMap<String, InsertedTranscript>,
}

impl PromptState {
    pub(crate) fn record_streamed_response(&mut self, key: String, text: String) {
        self.streamed_responses.insert(key, text);
    }

    pub(crate) fn remove_streamed_response(&mut self, key: &str) -> Option<String> {
        self.streamed_responses.remove(key)
    }

    pub(crate) fn take_live_response(&mut self, key: &str) -> Option<InsertedTranscript> {
        self.live_response_ranges.remove(key)
    }

    pub(crate) fn insert_live_response(&mut self, key: String, inserted: InsertedTranscript) {
        self.live_response_ranges.insert(key, inserted);
    }

    pub(crate) fn take_live_compaction(&mut self, key: &str) -> Option<InsertedTranscript> {
        self.live_compaction_ranges.remove(key)
    }

    pub(crate) fn insert_live_compaction(&mut self, key: String, inserted: InsertedTranscript) {
        self.live_compaction_ranges.insert(key, inserted);
    }

    pub(crate) fn remove_prompt(&mut self, key: &str) -> PromptCleanup {
        self.streamed_responses.remove(key);
        PromptCleanup {
            live_response: self.live_response_ranges.remove(key),
            live_compaction: self.live_compaction_ranges.remove(key),
        }
    }
}

pub(crate) struct PromptCleanup {
    pub(crate) live_response: Option<InsertedTranscript>,
    pub(crate) live_compaction: Option<InsertedTranscript>,
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn streamed_responses_are_recorded_and_removed() {
        let mut state = PromptState::default();

        state.record_streamed_response("prompt".to_owned(), "text".to_owned());

        assert_eq!(
            state.remove_streamed_response("prompt").as_deref(),
            Some("text")
        );
        assert_eq!(state.remove_streamed_response("prompt"), None);
    }

    #[test]
    fn remove_prompt_clears_streamed_response_even_when_no_live_ranges_exist() {
        let mut state = PromptState::default();
        state.record_streamed_response("prompt".to_owned(), "text".to_owned());

        let cleanup = state.remove_prompt("prompt");

        assert!(cleanup.live_response.is_none());
        assert!(cleanup.live_compaction.is_none());
        assert_eq!(state.remove_streamed_response("prompt"), None);
    }
}
