use std::collections::HashSet;

#[derive(Default)]
pub(crate) struct MainToolActivity {
    completed: u64,
    total: u64,
    visible: bool,
    backgrounded_tools: HashSet<String>,
}

impl MainToolActivity {
    pub(crate) fn add_requested_tools(&mut self, count: usize) {
        if count == 0 {
            return;
        }
        self.total = self.total.saturating_add(count as u64);
        self.visible = true;
    }

    pub(crate) fn record_backgrounded(&mut self, call_id: &str) {
        self.backgrounded_tools.insert(call_id.to_owned());
        if self.total != 0 {
            self.visible = true;
        }
    }

    pub(crate) fn record_completed(&mut self, call_id: &str) {
        self.backgrounded_tools.remove(call_id);
        if self.completed < self.total {
            self.completed += 1;
        }
        if self.total != 0 {
            self.visible = true;
        }
    }

    pub(crate) fn is_backgrounded(&self, call_id: &str) -> bool {
        self.backgrounded_tools.contains(call_id)
    }

    pub(crate) fn status_chip(&self) -> Option<String> {
        ((self.visible || !self.backgrounded_tools.is_empty()) && self.total != 0)
            .then(|| format!("{}/{}", self.completed, self.total))
    }

    pub(crate) fn reset(&mut self) {
        *self = Self::default();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn status_tracks_requested_backgrounded_and_completed_tools() {
        let mut activity = MainToolActivity::default();
        assert_eq!(activity.status_chip(), None);

        activity.add_requested_tools(2);
        assert_eq!(activity.status_chip().as_deref(), Some("0/2"));

        activity.record_backgrounded("call-1");
        assert!(activity.is_backgrounded("call-1"));
        assert_eq!(activity.status_chip().as_deref(), Some("0/2"));

        activity.record_completed("call-1");
        assert!(!activity.is_backgrounded("call-1"));
        assert_eq!(activity.status_chip().as_deref(), Some("1/2"));
    }

    #[test]
    fn reset_clears_visible_status() {
        let mut activity = MainToolActivity::default();
        activity.add_requested_tools(1);
        activity.record_backgrounded("call-1");

        activity.reset();

        assert_eq!(activity.status_chip(), None);
        assert!(!activity.is_backgrounded("call-1"));
    }
}
