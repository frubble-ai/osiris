//! GitHub, through our GitHub App: OAuth for the app's user token, and the actions done with it,
//! one folder per area of GitHub's API and one file per action. OAuth endpoints, action names,
//! inputs and scopes as in `OpenConnector` v1.9.4 (Apache-2.0), `src/providers/github/`.

pub mod get_page;
pub mod installations;
#[cfg(any(test, feature = "testing"))]
pub mod mocks;
pub mod paging;
pub mod path_segment;
pub mod provider;
pub mod repositories;
pub mod request;
pub mod revoke;
pub mod transformers;
pub mod types;
pub mod users;

#[cfg(test)]
mod tests;

pub use provider::Github;
