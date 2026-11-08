//! `github.watch_pull_request`'s configuration: the pull request it watches.

use super::repository_names::repository_names;
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

#[derive(Debug, Clone, Deserialize, Serialize, ToSchema)]
#[serde(deny_unknown_fields)]
pub struct WatchPullRequestConfig {
    pub owner: String,
    pub repo: String,
    /// The pull request's number.
    pub number: u64,
}

impl WatchPullRequestConfig {
    /// The configuration `value` holds, or why it isn't one.
    pub fn read(value: &serde_json::Value) -> Result<Self, String> {
        let config: Self = serde_json::from_value(value.clone())
            .map_err(|error| format!("the trigger's config: {error}"))?;
        repository_names(&config.owner, &config.repo)?;
        if config.number == 0 {
            return Err("config.number must be a pull request's number, from 1".into());
        }
        Ok(config)
    }
}
