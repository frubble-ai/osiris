//! Slack: OAuth for a user token (Slack's user-token flow, so the auth config's scopes are the
//! user's), and reads with it. Endpoints as in `OpenConnector` v1.9.4 (Apache-2.0),
//! `src/providers/slack/definition.ts`. Slack's Web API answers `ok: false` beside a 200.

pub mod conversations;
#[cfg(any(test, feature = "testing"))]
pub mod mocks;
pub mod provider;
pub mod registration;
pub mod revoke;
pub mod transformers;
pub mod types;

#[cfg(test)]
mod tests;

pub use provider::Slack;
