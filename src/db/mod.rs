//! The database: osiris's own, which nothing else reads or writes.

use crate::config::DatabaseConfig;
use sqlx::postgres::{PgPool, PgPoolOptions};

/// The migrations in `migrations/`, compiled into the binary.
static MIGRATOR: sqlx::migrate::Migrator = sqlx::migrate!("./migrations");

pub async fn connect(config: &DatabaseConfig) -> anyhow::Result<PgPool> {
    Ok(PgPoolOptions::new()
        .max_connections(10)
        .connect(&config.url)
        .await?)
}

/// Apply every pending migration; already applied ones are skipped.
pub async fn migrate(config: &DatabaseConfig) -> anyhow::Result<()> {
    let pool = connect(config).await?;
    MIGRATOR.run(&pool).await?;
    tracing::info!("migrations applied");
    Ok(())
}
