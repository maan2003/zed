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

pub(crate) fn left_chips(
    session_id: &tau_proto::SessionId,
    current_agent_id: Option<&str>,
    current_role: Option<&str>,
    current_model: Option<&tau_proto::ModelId>,
    baseline_params: Option<tau_proto::ModelParams>,
    current_params: tau_proto::ModelParams,
    role_default_effort: Option<tau_proto::Effort>,
    role_default_verbosity: Option<tau_proto::Verbosity>,
) -> Vec<Chip> {
    use tau_themes::names;

    let mut chips = Vec::new();
    chips.push(Chip::new(format!("&{session_id}"), names::STATUS_SESSION));
    match (current_agent_id, current_role, current_model) {
        (Some(agent_id), _, _) => chips.push(Chip::new(format!("@{agent_id}"), names::STATUS_ROLE)),
        (None, Some(role), _) => chips.push(Chip::new(format!("+{role}"), names::STATUS_ROLE)),
        (None, None, Some(model)) => {
            chips.push(Chip::new(format!("={model}"), names::STATUS_MODEL))
        }
        (None, None, None) => chips.push(Chip::new("no role selected", names::MODEL_STATUS)),
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

pub(crate) fn right_chips(
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
