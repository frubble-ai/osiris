//! `osiris connect <provider> --user org_1`: start a consent on the provider's app and open it in the
//! browser. With one app for the provider it is chosen; with several, name one with `--auth-config`.

pub mod choose_auth_config;
pub mod command;
pub mod default_callback_url;
pub mod start_consent;

pub use command::Connect;
