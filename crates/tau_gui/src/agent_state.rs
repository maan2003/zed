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

    pub(crate) fn observe_event(&mut self, event: &tau_proto::Event) {
        match event {
            tau_proto::Event::AgentStarted(started) => self.remember(started.agent_id.to_string()),
            tau_proto::Event::SessionAgentLoaded(loaded) => {
                self.remember(loaded.agent_id.to_string())
            }
            tau_proto::Event::SessionAgentUnloaded(unloaded) => {
                self.unload(unloaded.agent_id.as_str());
            }
            tau_proto::Event::UiPromptSubmitted(prompt) if prompt.originator.is_user() => {
                self.select(prompt.agent_id.to_string());
            }
            tau_proto::Event::AgentPromptSubmitted(prompt)
                if prompt.originator.is_user() && !prompt.message_class.is_internal() =>
            {
                self.select(prompt.agent_id.to_string());
            }
            tau_proto::Event::AgentPromptQueued(queued) if !queued.message_class.is_internal() => {
                self.select(queued.agent_id.to_string());
            }
            tau_proto::Event::AgentUserMessageInjected(injected)
                if !injected.message_class.is_internal() =>
            {
                self.remember(injected.agent_id.to_string());
            }
            tau_proto::Event::AgentMessageSent(message) => {
                self.remember(message.sender_id.to_string());
                if let Some(agent_id) = agent_message_sent_recipient_agent_id(message) {
                    self.remember(agent_id.to_owned());
                }
            }
            tau_proto::Event::AgentMessageReceived(message) => {
                self.remember(message.sender_id.to_string());
                self.remember(message.recipient_id.to_string());
            }
            tau_proto::Event::ToolDelegateProgress(progress) => {
                if let Some(agent_id) = &progress.agent_id {
                    self.mark_live(agent_id.clone());
                }
            }
            tau_proto::Event::AgentPromptCreated(created) if created.originator.is_user() => {
                self.select(created.agent_id.to_string());
            }
            tau_proto::Event::ProviderResponseFinished(finished)
                if finished.originator.is_user() =>
            {
                self.select(finished.agent_id.to_string());
            }
            _ => {}
        }
    }
}

fn agent_message_sent_recipient_agent_id(message: &tau_proto::AgentMessageSent) -> Option<&str> {
    match &message.recipient {
        tau_proto::AgentMessageRecipient::Agent { agent_id } => Some(agent_id.as_str()),
        tau_proto::AgentMessageRecipient::User => None,
    }
}
