//! Installs the process's logger, once, at startup.

use tracing_subscriber::{EnvFilter, fmt, fmt::format::JsonFields};

/// Installs the process's logger, once, at startup.
pub fn init() {
    let filter = EnvFilter::try_from_default_env().unwrap_or_else(|_| EnvFilter::new("info"));
    if std::env::var("LOG_FORMAT").as_deref() == Ok("json") {
        fmt()
            .with_env_filter(filter)
            .fmt_fields(JsonFields::new())
            .event_format(super::cloud_json::CloudJson)
            .init();
    } else {
        fmt().with_env_filter(filter).init();
    }
}
