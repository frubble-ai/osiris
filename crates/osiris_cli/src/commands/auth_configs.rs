//! `osiris auth-configs …`: the providers' apps.

pub mod create;
pub mod delete;
pub mod get;
pub mod list;
pub mod subcommand;
pub mod update;

pub use subcommand::AuthConfigs;
