//! Linear: OAuth for a user, and reads through its GraphQL API. Endpoints as in `OpenConnector`
//! v1.9.4 (Apache-2.0), `src/providers/linear/definition.ts`.

pub mod graphql;
#[cfg(any(test, feature = "testing"))]
pub mod mocks;
pub mod provider;
pub mod registration;
pub mod revoke;
pub mod teams;
pub mod transformers;
pub mod types;

#[cfg(test)]
mod tests;

pub use provider::Linear;
