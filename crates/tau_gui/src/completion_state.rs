use std::collections::HashSet;
use std::sync::{Arc, Mutex};

use editor::{CompletionContext, CompletionProvider, Editor};
use gpui::{Context, Entity, Task, Window};
use language::{Buffer, CodeLabel, ToOffset as _};
use project::{Completion, CompletionDisplayOptions, CompletionResponse, CompletionSource};

#[derive(Clone, Debug, Eq, PartialEq)]
pub(crate) struct CompletionCandidate {
    value: String,
    description: String,
}

impl CompletionCandidate {
    pub(crate) fn new(value: impl Into<String>, description: impl Into<String>) -> Self {
        Self {
            value: value.into(),
            description: description.into(),
        }
    }
}

#[derive(Default)]
pub(crate) struct TauCompletionState {
    roles: Vec<CompletionCandidate>,
    known_agents: Vec<String>,
    live_agents: HashSet<String>,
    suspended_agents: HashSet<String>,
}

impl TauCompletionState {
    pub(crate) fn set_roles(&mut self, roles: Vec<CompletionCandidate>) {
        self.roles = roles;
    }

    pub(crate) fn set_agents(
        &mut self,
        known_agents: Vec<String>,
        live_agents: HashSet<String>,
        suspended_agents: HashSet<String>,
    ) {
        self.known_agents = known_agents;
        self.live_agents = live_agents;
        self.suspended_agents = suspended_agents;
    }

    fn completions_for(&self, text_before_cursor: &str) -> Vec<CompletionCandidate> {
        if let Some(mention_prefix) = agent_mention_prefix(text_before_cursor) {
            return self.agent_mention_completions(mention_prefix);
        }
        if !text_before_cursor.starts_with('/') {
            return Vec::new();
        }

        let args = slash_command_args(text_before_cursor);
        match args.command.as_str() {
            "" => root_command_completions(&args.current),
            "/new" | "/compact" => Vec::new(),
            "/agent" => self.agent_command_completions(&args.arguments),
            "/model" => {
                candidate_matches(&self.roles, args.arguments.first().copied().unwrap_or(""))
            }
            "/role" => self.role_command_completions(&args.arguments),
            _ => Vec::new(),
        }
    }

    fn agent_mention_completions(&self, needle: &str) -> Vec<CompletionCandidate> {
        self.known_agents
            .iter()
            .filter(|agent| self.live_agents.contains(*agent))
            .filter(|agent| !self.suspended_agents.contains(*agent))
            .filter(|agent| completion_matches(agent, needle))
            .map(|agent| CompletionCandidate::new(agent, "agent"))
            .collect()
    }

    fn agent_command_completions(&self, args: &[&str]) -> Vec<CompletionCandidate> {
        match args.len() {
            0 | 1 => ["new", "switch", "suspend", "resume"]
                .into_iter()
                .filter(|subcommand| {
                    completion_matches(subcommand, args.first().copied().unwrap_or(""))
                })
                .map(|subcommand| CompletionCandidate::new(subcommand, "subcommand"))
                .collect(),
            2 => {
                let needle = args[1];
                agent_completion_candidates(
                    args[0],
                    &self.known_agents,
                    &self.live_agents,
                    &self.suspended_agents,
                )
                .into_iter()
                .filter(|agent| completion_matches(agent, needle))
                .map(|agent| CompletionCandidate::new(agent, "agent"))
                .collect()
            }
            _ => Vec::new(),
        }
    }

    fn role_command_completions(&self, args: &[&str]) -> Vec<CompletionCandidate> {
        match args.len() {
            0 | 1 => candidate_matches(&self.roles, args.first().copied().unwrap_or("")),
            2 => role_setting_completions(args[1]),
            3 => role_setting_value_completions(args[1], args[2]),
            _ => Vec::new(),
        }
    }
}

pub(crate) struct TauCompletionProvider {
    state: Arc<Mutex<TauCompletionState>>,
}

impl TauCompletionProvider {
    pub(crate) fn new(state: Arc<Mutex<TauCompletionState>>) -> Self {
        Self { state }
    }
}

impl CompletionProvider for TauCompletionProvider {
    fn completions(
        &self,
        buffer: &Entity<Buffer>,
        buffer_position: language::Anchor,
        _trigger: CompletionContext,
        _window: &mut Window,
        cx: &mut Context<Editor>,
    ) -> Task<anyhow::Result<Vec<CompletionResponse>>> {
        let buffer = buffer.read(cx);
        let cursor_offset = buffer_position.to_offset(&buffer);
        let start_anchor = buffer.anchor_before(0);
        let text_before_cursor: String = buffer
            .text_for_range(start_anchor..buffer_position)
            .collect();
        let replace_start = completion_replace_start(&text_before_cursor);
        let replace_range =
            buffer.anchor_before(replace_start)..buffer.anchor_before(cursor_offset);
        let completions = self
            .state
            .lock()
            .ok()
            .map(|state| state.completions_for(&text_before_cursor))
            .unwrap_or_default()
            .into_iter()
            .map(|candidate| Completion {
                replace_range: replace_range.clone(),
                new_text: candidate.value.clone(),
                label: CodeLabel::plain(candidate.value, None),
                documentation: None,
                source: CompletionSource::Custom,
                icon_path: None,
                match_start: None,
                snippet_deduplication_key: None,
                insert_text_mode: None,
                confirm: None,
                group: None,
            })
            .collect();

        Task::ready(Ok(vec![CompletionResponse {
            completions,
            display_options: CompletionDisplayOptions {
                dynamic_width: true,
            },
            is_incomplete: false,
        }]))
    }

    fn is_completion_trigger(
        &self,
        _buffer: &Entity<Buffer>,
        _position: language::Anchor,
        text: &str,
        _trigger_in_words: bool,
        _cx: &mut Context<Editor>,
    ) -> bool {
        text.chars().last().is_some_and(|character| {
            character == '/'
                || character == '@'
                || character == ' '
                || character == '-'
                || character.is_ascii_alphanumeric()
        })
    }
}

struct SlashCommandArgs<'a> {
    command: String,
    arguments: Vec<&'a str>,
    current: String,
}

fn slash_command_args(text: &str) -> SlashCommandArgs<'_> {
    let has_trailing_space = text.ends_with(char::is_whitespace);
    let mut parts = text.split_whitespace().collect::<Vec<_>>();
    if parts.len() <= 1 && !has_trailing_space {
        return SlashCommandArgs {
            command: String::new(),
            arguments: Vec::new(),
            current: text.to_owned(),
        };
    }
    if has_trailing_space {
        parts.push("");
    }
    let command = parts.first().copied().unwrap_or("").to_owned();
    let arguments = parts.into_iter().skip(1).collect::<Vec<_>>();
    let current = arguments.last().copied().unwrap_or("").to_owned();
    SlashCommandArgs {
        command,
        arguments,
        current,
    }
}

fn completion_replace_start(text_before_cursor: &str) -> usize {
    text_before_cursor
        .char_indices()
        .rev()
        .find_map(|(index, character)| {
            character
                .is_whitespace()
                .then_some(index + character.len_utf8())
        })
        .unwrap_or(0)
}

fn root_command_completions(needle: &str) -> Vec<CompletionCandidate> {
    [
        CompletionCandidate::new("/agent", "Manage visible/suspended agent transcripts"),
        CompletionCandidate::new("/new", "Alias for /agent new"),
        CompletionCandidate::new("/model", "Select an agent role"),
        CompletionCandidate::new("/role", "Switch, create, edit, or delete an agent role"),
        CompletionCandidate::new("/compact", "Force a compaction pass"),
    ]
    .into_iter()
    .filter(|candidate| completion_matches(&candidate.value, needle))
    .collect()
}

fn role_setting_completions(needle: &str) -> Vec<CompletionCandidate> {
    [
        ("delete", "delete role"),
        ("model", "model id"),
        ("effort", "reasoning effort"),
        ("verbosity", "response verbosity"),
        ("thinking-summary", "thinking summary"),
        ("service-tier", "service tier"),
        ("compaction-threshold", "compaction threshold"),
        ("tools", "enabled tools"),
        ("enable-tools", "enable tools"),
        ("disable-tools", "disable tools"),
    ]
    .into_iter()
    .filter(|(value, _)| completion_matches(value, needle))
    .map(|(value, description)| CompletionCandidate::new(value, description))
    .collect()
}

fn role_setting_value_completions(setting: &str, needle: &str) -> Vec<CompletionCandidate> {
    let values: &[(&str, &str)] = match setting {
        "model" | "tools" | "enable-tools" | "disable-tools" => &[("reset", "clear override")],
        "effort" => &[
            ("reset", "clear override"),
            ("off", "disable reasoning"),
            ("minimal", "minimal"),
            ("low", "low"),
            ("medium", "medium"),
            ("high", "high"),
            ("xhigh", "extra high"),
        ],
        "verbosity" => &[
            ("reset", "clear override"),
            ("low", "low"),
            ("medium", "medium"),
            ("high", "high"),
        ],
        "thinking-summary" => &[
            ("reset", "clear override"),
            ("off", "off"),
            ("auto", "auto"),
            ("concise", "concise"),
            ("detailed", "detailed"),
        ],
        "service-tier" => &[
            ("reset", "clear override"),
            ("fast", "fast"),
            ("flex", "flex"),
        ],
        _ => &[],
    };
    values
        .iter()
        .copied()
        .filter(|(value, _)| completion_matches(value, needle))
        .map(|(value, description)| CompletionCandidate::new(value, description))
        .collect()
}

fn agent_completion_candidates(
    subcommand: &str,
    known_agents: &[String],
    live_agents: &HashSet<String>,
    suspended_agents: &HashSet<String>,
) -> Vec<String> {
    let active_agents = live_agents
        .difference(suspended_agents)
        .cloned()
        .collect::<HashSet<_>>();
    match subcommand {
        "switch" => {
            let mut agents = known_agents
                .iter()
                .filter(|agent| active_agents.contains(*agent))
                .cloned()
                .collect::<Vec<_>>();
            agents.insert(0, "none".to_owned());
            agents
        }
        "suspend" => known_agents
            .iter()
            .filter(|agent| active_agents.contains(*agent))
            .cloned()
            .collect(),
        "resume" => known_agents
            .iter()
            .filter(|agent| !active_agents.contains(*agent))
            .cloned()
            .collect(),
        _ => Vec::new(),
    }
}

fn candidate_matches(candidates: &[CompletionCandidate], needle: &str) -> Vec<CompletionCandidate> {
    candidates
        .iter()
        .filter(|candidate| completion_matches(&candidate.value, needle))
        .cloned()
        .collect()
}

fn completion_matches(value: &str, needle: &str) -> bool {
    let needle = needle.to_lowercase();
    needle.is_empty() || value.to_lowercase().contains(&needle)
}

fn agent_mention_prefix(text: &str) -> Option<&str> {
    let token_start = completion_replace_start(text);
    let token = text.get(token_start..)?;
    token.strip_prefix('@')
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn agent_completions_match_cli_subcommands_and_filters() {
        let mut state = TauCompletionState::default();
        state.set_agents(
            vec!["helper".to_owned(), "worker".to_owned()],
            HashSet::from(["helper".to_owned(), "worker".to_owned()]),
            HashSet::from(["helper".to_owned()]),
        );

        let subcommands = state
            .completions_for("/agent ")
            .into_iter()
            .map(|candidate| candidate.value)
            .collect::<Vec<_>>();
        assert_eq!(subcommands, vec!["new", "switch", "suspend", "resume"]);

        let switch_agents = state
            .completions_for("/agent switch ")
            .into_iter()
            .map(|candidate| candidate.value)
            .collect::<Vec<_>>();
        assert_eq!(switch_agents, vec!["none", "worker"]);

        let resume_agents = state
            .completions_for("/agent resume ")
            .into_iter()
            .map(|candidate| candidate.value)
            .collect::<Vec<_>>();
        assert_eq!(resume_agents, vec!["helper"]);
    }

    #[test]
    fn role_completions_offer_role_settings_and_values() {
        let mut state = TauCompletionState::default();
        state.set_roles(vec![CompletionCandidate::new("senior-engineer", "role")]);

        assert_eq!(
            state.completions_for("/model senior")[0].value,
            "senior-engineer"
        );
        assert_eq!(
            state.completions_for("/role senior")[0].value,
            "senior-engineer"
        );
        assert!(
            state
                .completions_for("/role senior-engineer effort ")
                .iter()
                .any(|candidate| candidate.value == "high")
        );
    }

    #[test]
    fn agent_mentions_complete_active_agents() {
        let mut state = TauCompletionState::default();
        state.set_agents(
            vec!["helper".to_owned(), "worker".to_owned()],
            HashSet::from(["helper".to_owned(), "worker".to_owned()]),
            HashSet::from(["helper".to_owned()]),
        );

        let mentions = state
            .completions_for("ask @w")
            .into_iter()
            .map(|candidate| candidate.value)
            .collect::<Vec<_>>();
        assert_eq!(mentions, vec!["worker"]);
    }
}
