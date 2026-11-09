//! `github.on_repo_event`'s configuration: the repository and the webhook events it fires on.

use super::repository_names::repository_names;
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

/// The webhook events GitHub sends a repository's webhook, as `OpenConnector` lists them, and `*`
/// for every one.
pub const EVENTS: &[&str] = &[
    "*",
    "branch_protection_configuration",
    "branch_protection_rule",
    "check_run",
    "check_suite",
    "code_scanning_alert",
    "commit_comment",
    "create",
    "custom_property_values",
    "delete",
    "dependabot_alert",
    "deploy_key",
    "deployment",
    "deployment_status",
    "discussion",
    "discussion_comment",
    "fork",
    "gollum",
    "issue_comment",
    "issue_dependencies",
    "issues",
    "label",
    "member",
    "meta",
    "milestone",
    "package",
    "page_build",
    "project",
    "project_card",
    "project_column",
    "public",
    "pull_request",
    "pull_request_review",
    "pull_request_review_comment",
    "pull_request_review_thread",
    "push",
    "registry_package",
    "release",
    "repository",
    "repository_advisory",
    "repository_import",
    "repository_ruleset",
    "repository_vulnerability_alert",
    "secret_scanning_alert",
    "secret_scanning_alert_location",
    "secret_scanning_scan",
    "security_and_analysis",
    "star",
    "status",
    "sub_issues",
    "team_add",
    "watch",
    "workflow_job",
    "workflow_run",
];

#[derive(Debug, Clone, Deserialize, Serialize, ToSchema)]
#[serde(deny_unknown_fields)]
pub struct RepoEventConfig {
    pub owner: String,
    pub repo: String,
    /// The webhook events it fires on, at least one, or `["*"]` for every one.
    pub events: Vec<String>,
}

impl RepoEventConfig {
    /// The configuration `value` holds, or why it isn't one.
    pub fn read(value: &serde_json::Value) -> Result<Self, String> {
        let config: Self = serde_json::from_value(value.clone())
            .map_err(|error| format!("the trigger's config: {error}"))?;
        repository_names(&config.owner, &config.repo)?;
        if config.events.is_empty() {
            return Err("config.events must name at least one event".into());
        }
        if let Some(unknown) = config
            .events
            .iter()
            .find(|event| !EVENTS.contains(&event.as_str()))
        {
            return Err(format!(
                "config.events: {unknown:?} is not a GitHub webhook event"
            ));
        }
        Ok(config)
    }
}
