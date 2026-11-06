//! GitHub, through our GitHub App: OAuth for the app's user token, and reads with it. The token
//! reads only what the app is installed on, so the reads are the app's installations and the
//! repositories they give. OAuth endpoints as in `OpenConnector` v1.9.4 (Apache-2.0),
//! `src/providers/github/definition.ts`.

pub mod get_page;
pub mod list_installations;
pub mod list_repositories;
#[cfg(any(test, feature = "testing"))]
pub mod mocks;
pub mod provider;
pub mod revoke;
pub mod transformers;
pub mod types;

#[cfg(test)]
mod tests;

pub use provider::Github;
