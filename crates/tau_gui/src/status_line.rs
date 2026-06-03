pub(crate) struct AgentTab {
    pub(crate) agent_id: String,
    pub(crate) label: String,
    pub(crate) selected: bool,
    pub(crate) suspended: bool,
}

impl AgentTab {
    pub(crate) fn new(agent_id: String, selected: bool, suspended: bool) -> Self {
        let status_suffix = if suspended { ":paused" } else { "" };
        Self {
            label: format!("@{agent_id}{status_suffix} "),
            agent_id,
            selected,
            suspended,
        }
    }
}

pub(crate) struct StatusLine {
    pub(crate) agent_tabs: Vec<AgentTab>,
    pub(crate) left_chips: Vec<Chip>,
    pub(crate) right_chips: Vec<Chip>,
}

pub(crate) struct StatusLineInput<'a> {
    pub(crate) agent_tabs: Vec<AgentTab>,
    pub(crate) current_role: Option<&'a str>,
    pub(crate) current_model: Option<&'a tau_proto::ModelId>,
    pub(crate) baseline_params: Option<tau_proto::ModelParams>,
    pub(crate) current_params: tau_proto::ModelParams,
    pub(crate) role_default_effort: Option<tau_proto::Effort>,
    pub(crate) role_default_verbosity: Option<tau_proto::Verbosity>,
    pub(crate) main_tools_status: Option<String>,
    pub(crate) active_agents: usize,
    pub(crate) context_status: Option<String>,
}

pub(crate) fn build(input: StatusLineInput<'_>) -> StatusLine {
    let left_chips = left_chips(
        left_identity(&input),
        input.baseline_params,
        input.current_params,
        input.role_default_effort,
        input.role_default_verbosity,
    );
    let right_chips = right_chips(
        input.main_tools_status,
        input.active_agents,
        input.context_status,
    );

    StatusLine {
        agent_tabs: input.agent_tabs,
        left_chips,
        right_chips,
    }
}

pub(crate) struct Chip {
    pub(crate) text: String,
    pub(crate) style_name: &'static str,
}

impl Chip {
    fn new(text: impl Into<String>, style_name: &'static str) -> Self {
        Self {
            text: text.into(),
            style_name,
        }
    }
}

enum LeftStatusIdentity<'a> {
    Role(&'a str),
    Model(&'a tau_proto::ModelId),
    NoRoleSelected,
}

fn left_identity<'a>(input: &'a StatusLineInput<'a>) -> Option<LeftStatusIdentity<'a>> {
    if input.agent_tabs.iter().any(|agent| agent.selected) {
        None
    } else if let Some(role) = input.current_role {
        Some(LeftStatusIdentity::Role(role))
    } else if let Some(model) = input.current_model {
        Some(LeftStatusIdentity::Model(model))
    } else {
        Some(LeftStatusIdentity::NoRoleSelected)
    }
}

fn left_chips(
    identity: Option<LeftStatusIdentity<'_>>,
    baseline_params: Option<tau_proto::ModelParams>,
    current_params: tau_proto::ModelParams,
    role_default_effort: Option<tau_proto::Effort>,
    role_default_verbosity: Option<tau_proto::Verbosity>,
) -> Vec<Chip> {
    use tau_themes::names;

    let mut chips = Vec::new();
    match identity {
        Some(LeftStatusIdentity::Role(role)) => {
            chips.push(Chip::new(format!("+{role}"), names::STATUS_ROLE))
        }
        Some(LeftStatusIdentity::Model(model)) => {
            chips.push(Chip::new(format!("={model}"), names::STATUS_MODEL))
        }
        Some(LeftStatusIdentity::NoRoleSelected) => {
            chips.push(Chip::new("no role selected", names::MODEL_STATUS))
        }
        None => {}
    }
    if show_effort_status(baseline_params, current_params, role_default_effort) {
        chips.push(Chip::new(
            format!("^{}", current_params.effort.as_str()),
            names::STATUS_EFFORT,
        ));
    }
    if show_verbosity_status(baseline_params, current_params, role_default_verbosity) {
        chips.push(Chip::new(
            format!("~{}", current_params.verbosity.as_str()),
            names::STATUS_VERBOSITY,
        ));
    }
    if show_service_tier_status(baseline_params, current_params) {
        let service_tier = current_params
            .service_tier
            .map(|tier| tier.as_str())
            .unwrap_or("off");
        chips.push(Chip::new(
            format!("!{service_tier}"),
            names::STATUS_SERVICE_TIER,
        ));
    }
    chips
}

fn right_chips(
    main_tools_status: Option<String>,
    active_agents: usize,
    context_status: Option<String>,
) -> Vec<Chip> {
    use tau_themes::names;

    let mut chips = Vec::new();
    if let Some(tools) = main_tools_status {
        chips.push(Chip::new(format!("%{tools}"), names::STATUS_TOOLS));
    }
    if active_agents > 0 {
        chips.push(Chip::new(format!("@{active_agents}"), names::STATUS_AGENTS));
    }
    if let Some(context) = context_status {
        chips.push(Chip::new(format!("#{context}"), names::STATUS_CONTEXT));
    }
    chips
}

fn show_effort_status(
    baseline_params: Option<tau_proto::ModelParams>,
    current_params: tau_proto::ModelParams,
    role_default_effort: Option<tau_proto::Effort>,
) -> bool {
    baseline_params.map_or_else(
        || {
            role_default_effort.map_or(!current_params.effort.is_default(), |default| {
                current_params.effort != default
            })
        },
        |default| current_params.effort != default.effort,
    )
}

fn show_verbosity_status(
    baseline_params: Option<tau_proto::ModelParams>,
    current_params: tau_proto::ModelParams,
    role_default_verbosity: Option<tau_proto::Verbosity>,
) -> bool {
    baseline_params.map_or_else(
        || {
            role_default_verbosity.map_or(!current_params.verbosity.is_default(), |default| {
                current_params.verbosity != default
            })
        },
        |default| current_params.verbosity != default.verbosity,
    )
}

fn show_service_tier_status(
    baseline_params: Option<tau_proto::ModelParams>,
    current_params: tau_proto::ModelParams,
) -> bool {
    baseline_params.map_or(current_params.service_tier.is_some(), |default| {
        current_params.service_tier != default.service_tier
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    fn input(agent_tabs: Vec<AgentTab>) -> StatusLineInput<'static> {
        StatusLineInput {
            agent_tabs,
            current_role: Some("senior-engineer"),
            current_model: None,
            baseline_params: None,
            current_params: tau_proto::ModelParams::default(),
            role_default_effort: None,
            role_default_verbosity: None,
            main_tools_status: None,
            active_agents: 0,
            context_status: None,
        }
    }

    #[test]
    fn build_hides_primary_identity_when_selected_agent_tab_exists() {
        let status_line = build(input(vec![AgentTab::new(
            "agent-a".to_owned(),
            true,
            false,
        )]));

        assert_eq!(status_line.agent_tabs[0].label, "@agent-a ");
        assert!(status_line.left_chips.is_empty());
    }

    #[test]
    fn build_renders_role_identity_without_selected_agent_tab() {
        let status_line = build(input(vec![AgentTab::new(
            "agent-a".to_owned(),
            false,
            false,
        )]));

        assert_eq!(status_line.left_chips[0].text, "+senior-engineer");
        assert_eq!(
            status_line.left_chips[0].style_name,
            tau_themes::names::STATUS_ROLE
        );
    }

    #[test]
    fn left_chips_can_omit_primary_identity() {
        let chips = left_chips(None, None, tau_proto::ModelParams::default(), None, None);

        assert!(
            chips.iter().all(|chip| chip.text != "no role selected"),
            "hidden active-agent identity must not fall through to no-role status"
        );
    }

    #[test]
    fn left_chips_render_no_role_when_identity_says_so() {
        let chips = left_chips(
            Some(LeftStatusIdentity::NoRoleSelected),
            None,
            tau_proto::ModelParams::default(),
            None,
            None,
        );

        assert_eq!(chips[0].text, "no role selected");
        assert_eq!(chips[0].style_name, tau_themes::names::MODEL_STATUS);
    }

    #[test]
    fn right_chips_render_tool_agent_and_context_status() {
        let chips = right_chips(Some("1/2".to_owned()), 1, Some("50%".to_owned()));

        assert_eq!(chips.len(), 3);
        assert_eq!(chips[0].text, "%1/2");
        assert_eq!(chips[0].style_name, tau_themes::names::STATUS_TOOLS);
        assert_eq!(chips[1].text, "@1");
        assert_eq!(chips[1].style_name, tau_themes::names::STATUS_AGENTS);
        assert_eq!(chips[2].text, "#50%");
        assert_eq!(chips[2].style_name, tau_themes::names::STATUS_CONTEXT);
    }

    #[test]
    fn right_chips_hide_zero_active_agents() {
        let chips = right_chips(None, 0, None);

        assert!(chips.is_empty());
    }
}
