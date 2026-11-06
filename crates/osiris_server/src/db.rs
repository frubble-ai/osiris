//! The database connection, and the migration job. The pool opens its first connection at
//! startup, so a database that can't be reached, or whose TLS can't be established against
//! `DB_CA_PEM`, stops the process right away instead of failing the first request.

use crate::config::DatabaseConfig;
use anyhow::Context;
use sqlx::postgres::{PgPool, PgPoolOptions};

/// The migrations in `migrations/`, compiled in.
pub static MIGRATOR: sqlx::migrate::Migrator = sqlx::migrate!("../../migrations");

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

/// Apply every pending migration; already applied ones are skipped.
pub async fn migrate(config: &DatabaseConfig) -> anyhow::Result<()> {
    let pool = connect(config).await?;
    MIGRATOR
        .run(&pool)
        .await
        .context("applying the migrations")?;
    tracing::info!("migrations applied");
    Ok(())
}
