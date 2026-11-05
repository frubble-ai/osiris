//! Logs to stdout: with `LOG_FORMAT=json`, one JSON object per line that Cloud Logging reads,
//! severity included (see `cloud_json`); otherwise readable lines for a terminal. `RUST_LOG`
//! filters them (`info` when unset).

pub mod cloud_json;

use tracing_subscriber::{EnvFilter, fmt, fmt::format::JsonFields};

/// Installs the process's logger, once, at startup.
pub fn init() {
    let filter = EnvFilter::try_from_default_env().unwrap_or_else(|_| EnvFilter::new("info"));
    if std::env::var("LOG_FORMAT").as_deref() == Ok("json") {
        fmt()
            .with_env_filter(filter)
            .fmt_fields(JsonFields::new())
            .event_format(cloud_json::CloudJson)
            .init();
    } else {
        fmt().with_env_filter(filter).init();
    }
}
