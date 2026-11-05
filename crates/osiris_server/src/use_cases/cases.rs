//! Every use case, one file each: each adds one method to [`UseCases`](super::UseCases).

pub mod abandon_idempotent_request;
pub mod begin_idempotent_request;
pub mod check_database;
pub mod connected_account_token;
pub mod create_auth_config;
pub mod delete_auth_config;
pub mod delete_connected_account;
pub mod expire_consent;
pub mod finish_consent;
pub mod finish_idempotent_request;
pub mod get_auth_config;
pub mod get_connected_account;
pub mod list_auth_configs;
pub mod list_connected_accounts;
pub mod list_github_installations;
pub mod list_github_repositories;
pub mod list_linear_teams;
pub mod list_slack_channels;
pub mod publish_event_retry;
pub mod refresh_connected_account;
pub mod refresh_connected_account_now;
pub mod revoke_connected_account;
pub mod run_connected_account_action;
pub mod run_refresh_task;
pub mod run_task;
pub mod start_connect;
pub mod trigger_event;
pub mod update_auth_config;

#[cfg(test)]
mod tests;
