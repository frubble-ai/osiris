//! Logs to stdout: with `LOG_FORMAT=json`, one JSON object per line that Cloud Logging reads,
//! severity included (see `cloud_json`); otherwise readable lines for a terminal. `RUST_LOG`
//! filters them (`info` when unset).

pub mod cloud_json;
pub mod init;

pub use init::init;
