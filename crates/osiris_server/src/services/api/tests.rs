//! The API's HTTP contract, through its router: statuses, error codes, redirects, keys, versions
//! and idempotency, by domain.

mod actions;
mod auth_configs;
mod connecting;
mod disconnecting;
mod events;
mod github_actions;
mod golden;
mod health;
mod idempotency;
mod installations;
mod keys_and_versions;
mod listing;
mod refreshing;
mod releases;
mod unreadable_requests;
