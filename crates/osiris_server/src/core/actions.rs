//! Running a provider's actions with a connection: a fresh token first, the answer typed by the
//! provider's module, and a rejected token handled the same way for every action.

pub mod github;
pub mod linear;
pub mod revoked;
pub mod slack;
pub mod token_for;
pub mod with_token;
