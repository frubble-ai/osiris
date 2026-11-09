//! Acme: OAuth for a user, and reads with the token. Copy this folder to
//! `crates/osiris_providers/src/`, and `actions/acme/` to `scripts/actions/acme/`, rename
//! `acme`/`Acme`, then follow docs/add_provider.md.

pub mod provider;
pub mod registration;
pub mod revoke;
pub mod transformers;
pub mod types;
pub mod widgets;

pub use provider::Acme;
