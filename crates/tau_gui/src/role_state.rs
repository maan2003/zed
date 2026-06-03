use std::collections::HashMap;

#[derive(Clone, Debug, Default, Eq, PartialEq)]
struct RoleDetails {
    effort: Option<String>,
    verbosity: Option<String>,
}

#[derive(Default)]
pub(crate) struct RoleState {
    roles: Vec<String>,
    details: HashMap<String, RoleDetails>,
}

impl RoleState {
    pub(crate) fn update_available(&mut self, roles: &tau_proto::HarnessRolesAvailable) {
        self.roles = roles.roles.iter().map(|role| role.name.clone()).collect();
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

    pub(crate) fn role_by_delta(&self, current: Option<&str>, delta: isize) -> Option<String> {
        let roles = self.roles.as_slice();
        if roles.is_empty() {
            return None;
        }

        let index = current
            .and_then(|current| roles.iter().position(|role| role == current))
            .map(|index| (index as isize + delta).rem_euclid(roles.len() as isize) as usize)
            .unwrap_or_else(|| if delta < 0 { roles.len() - 1 } else { 0 });
        roles.get(index).cloned()
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

    fn roles_available(names: &[&str]) -> tau_proto::HarnessRolesAvailable {
        tau_proto::HarnessRolesAvailable {
            roles: names
                .iter()
                .map(|name| tau_proto::HarnessRoleInfo {
                    name: (*name).to_owned(),
                    description: "model=test/model".to_owned(),
                    role_description: None,
                })
                .collect(),
            groups: Vec::new(),
        }
    }

    #[test]
    fn cycles_roles_in_advertised_order() {
        let mut roles = RoleState::default();
        roles.update_available(&roles_available(&["engineer", "reviewer", "writer"]));

        assert_eq!(
            roles.role_by_delta(Some("engineer"), 1).as_deref(),
            Some("reviewer")
        );
        assert_eq!(
            roles.role_by_delta(Some("engineer"), -1).as_deref(),
            Some("writer")
        );
        assert_eq!(
            roles.role_by_delta(Some("writer"), 1).as_deref(),
            Some("engineer")
        );
    }

    #[test]
    fn cycles_from_list_edges_when_current_role_is_unknown() {
        let mut roles = RoleState::default();
        roles.update_available(&roles_available(&["engineer", "reviewer"]));

        assert_eq!(roles.role_by_delta(None, 1).as_deref(), Some("engineer"));
        assert_eq!(roles.role_by_delta(None, -1).as_deref(), Some("reviewer"));
        assert_eq!(
            roles.role_by_delta(Some("missing"), 1).as_deref(),
            Some("engineer")
        );
    }

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
