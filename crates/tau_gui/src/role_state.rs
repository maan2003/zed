use std::collections::HashMap;

#[derive(Clone, Debug, Default, Eq, PartialEq)]
struct RoleDetails {
    effort: Option<String>,
    verbosity: Option<String>,
}

#[derive(Default)]
pub(crate) struct RoleState {
    details: HashMap<String, RoleDetails>,
}

impl RoleState {
    pub(crate) fn update_available(&mut self, roles: &tau_proto::HarnessRolesAvailable) {
        self.details = roles
            .roles
            .iter()
            .map(|role| {
                (
                    role.name.clone(),
                    RoleDetails::from_description(&role.description),
                )
            })
            .collect();
    }

    pub(crate) fn default_effort(&self, role: Option<&str>) -> Option<tau_proto::Effort> {
        self.details.get(role?)?.effort.as_deref()?.parse().ok()
    }

    pub(crate) fn default_verbosity(&self, role: Option<&str>) -> Option<tau_proto::Verbosity> {
        self.details.get(role?)?.verbosity.as_deref()?.parse().ok()
    }
}

impl RoleDetails {
    fn from_description(description: &str) -> Self {
        let mut details = Self::default();
        if description == "no model" {
            return details;
        }

        for part in description.split(',').map(str::trim) {
            let Some((key, value)) = part.split_once('=') else {
                continue;
            };
            match key {
                "effort" => details.effort = Some(value.to_owned()),
                "verbosity" => details.verbosity = Some(value.to_owned()),
                _ => {}
            }
        }
        details
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_effort_and_verbosity_defaults_from_role_descriptions() {
        let details = RoleDetails::from_description(
            "model=gpt-5, effort=high, verbosity=low, service-tier=fast",
        );

        assert_eq!(details.effort.as_deref(), Some("high"));
        assert_eq!(details.verbosity.as_deref(), Some("low"));
    }

    #[test]
    fn ignores_no_model_descriptions() {
        let details = RoleDetails::from_description("no model");

        assert_eq!(details, RoleDetails::default());
    }
}
