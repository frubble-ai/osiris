//! The API's HTTP contract, through its router: statuses, error codes, redirects, keys, versions
//! and idempotency, by domain. Each provider's own routes are tested in its module.

mod auth_configs;
mod connecting;
mod disconnecting;
mod events;
mod github;
mod golden;
mod health;
mod idempotency;
mod keys_and_versions;
mod linear;
mod listing;
mod refreshing;
mod releases;
mod slack;
mod unreadable_requests;
