use std::collections::{HashMap, HashSet};

#[derive(Clone, Copy)]
pub(crate) struct AgentContextUsage {
    pub(crate) input_tokens: Option<u64>,
    pub(crate) percent_used: Option<u8>,
    pub(crate) context_window: Option<u64>,
}

#[derive(Default)]
pub(crate) struct AgentState {
    current_agent_id: Option<String>,
    known_agents: HashSet<String>,
    live_agents: HashSet<String>,
    suspended_agents: HashSet<String>,
    context_usage: HashMap<String, AgentContextUsage>,
}

impl AgentState {
    pub(crate) fn current_agent_id(&self) -> Option<&str> {
        self.current_agent_id.as_deref()
    }

    pub(crate) fn current_agent_id_owned(&self) -> Option<String> {
        self.current_agent_id.clone()
    }

    pub(crate) fn clear_current_agent(&mut self) {
        self.current_agent_id = None;
    }

    pub(crate) fn remember(&mut self, agent_id: impl Into<String>) {
        self.known_agents.insert(agent_id.into());
    }

    pub(crate) fn mark_live(&mut self, agent_id: impl Into<String>) {
        let agent_id = agent_id.into();
        self.known_agents.insert(agent_id.clone());
        self.live_agents.insert(agent_id.clone());
        self.suspended_agents.remove(&agent_id);
    }

    pub(crate) fn select(&mut self, agent_id: impl Into<String>) {
        let agent_id = agent_id.into();
        self.known_agents.insert(agent_id.clone());
        self.live_agents.insert(agent_id.clone());
        self.suspended_agents.remove(&agent_id);
        if self.current_agent_id.as_deref() != Some(agent_id.as_str()) {
            self.current_agent_id = Some(agent_id);
        }
    }

    pub(crate) fn unload(&mut self, agent_id: &str) {
        self.live_agents.remove(agent_id);
        self.suspended_agents.remove(agent_id);
        if self.current_agent_id.as_deref() == Some(agent_id) {
            self.current_agent_id = None;
        }
    }

    pub(crate) fn known(&self, agent_id: &str) -> bool {
        self.known_agents.contains(agent_id)
    }

    pub(crate) fn suspended(&self, agent_id: &str) -> bool {
        self.suspended_agents.contains(agent_id)
    }

    pub(crate) fn selected_is_active(&self) -> bool {
        let Some(agent_id) = self.current_agent_id.as_deref() else {
            return true;
        };
        self.live_agents.contains(agent_id) && !self.suspended_agents.contains(agent_id)
    }

    pub(crate) fn known_agents_sorted(&self) -> Vec<String> {
        let mut known_agents = self.known_agents.iter().cloned().collect::<Vec<_>>();
        known_agents.sort();
        known_agents
    }

    pub(crate) fn active_count(&self) -> usize {
        self.live_agents.difference(&self.suspended_agents).count()
    }

    pub(crate) fn active_side_count(&self) -> usize {
        self.live_agents
            .iter()
            .filter(|agent_id| {
                self.current_agent_id.as_deref() != Some(agent_id.as_str())
                    && !self.suspended_agents.contains(agent_id.as_str())
            })
            .count()
    }

    pub(crate) fn suspend(&mut self, agent_id: String) {
        self.suspended_agents.insert(agent_id);
    }

    pub(crate) fn resume(&mut self, agent_id: String) {
        self.live_agents.insert(agent_id.clone());
        self.suspended_agents.remove(&agent_id);
        self.current_agent_id = Some(agent_id);
    }

    pub(crate) fn record_context_usage(&mut self, agent_id: String, usage: AgentContextUsage) {
        self.context_usage.insert(agent_id, usage);
    }

    pub(crate) fn selected_context_usage(&self) -> Option<AgentContextUsage> {
        self.current_agent_id
            .as_deref()
            .and_then(|agent_id| self.context_usage.get(agent_id).copied())
    }

    pub(crate) fn clear_context_usage(&mut self) {
        self.context_usage.clear();
    }
}
