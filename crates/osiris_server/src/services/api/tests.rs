//! The API's HTTP contract, through its router: statuses, error codes, redirects, keys, versions
//! and idempotency, by domain.

mod actions;
mod auth_configs;
mod connecting;
mod disconnecting;
mod events;
mod health;
mod idempotency;
mod keys_and_versions;
