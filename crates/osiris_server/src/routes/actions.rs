//! `/v1/connected-accounts/{id}/actions/{action}`: one route per action, so each answer has its
//! own type in the spec. API key.

pub mod github;
pub mod linear;
pub mod router;
pub mod slack;

pub use router::router;
