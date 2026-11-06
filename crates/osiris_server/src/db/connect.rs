use crate::config::DatabaseConfig;
use anyhow::Context;
use sqlx::postgres::{PgPool, PgPoolOptions};

/// Opens the pool. The first connection is made here, so a database that cannot be
/// reached stops the process at startup.
pub async fn connect(config: &DatabaseConfig) -> anyhow::Result<PgPool> {
    PgPoolOptions::new()
        .max_connections(config.max_connections)
        .acquire_timeout(config.connect_timeout)
        .connect_with(config.connect_options())
        .await
        .with_context(|| {
            format!(
                "connecting to the database at {}:{} over TLS (verify-ca against DB_CA_PEM)",
                config.host, config.port
            )
        })
}
