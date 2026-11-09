//! GitHub's triggers, as `OpenConnector`'s (v1.9.4, Apache-2.0,
//! `src/providers/github/trigger-*.ts`): `github.on_repo_event` fires on a repository's webhook
//! events, and `github.watch_pull_request` on changes to one pull request, seen in its webhook
//! deliveries and in checks on a schedule. Both are a repository webhook osiris creates, signed
//! with the trigger's own secret.

pub mod check_pull_request;
pub mod create_hook;
pub mod delete_hook;
pub mod find_hook;
pub mod hook_body;
pub mod kinds;
pub mod receive_delivery;
pub mod repo_event_config;
pub mod repository_names;
pub mod verify_signature;
pub mod watch_pull_request_config;

pub use kinds::{ON_REPO_EVENT, WATCH_PULL_REQUEST};
pub use repo_event_config::RepoEventConfig;
pub use watch_pull_request_config::WatchPullRequestConfig;

#[cfg(test)]
mod tests;
