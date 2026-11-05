//! `auth_config`: the providers' apps, their secret encrypted with osiris's own data key. One
//! operation per module.

pub mod create;
pub mod delete;
pub mod find;
pub mod list;
pub mod types;
pub mod update;

pub use types::{AuthConfigChanges, NewAuthConfig};

#[cfg(test)]
mod tests;
