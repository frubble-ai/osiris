use super::MIGRATOR;
use anyhow::Context;
use sqlx::PgPool;

pub(super) async fn apply(pool: &PgPool) -> anyhow::Result<()> {
    MIGRATOR
        .run(pool)
        .await
        .context("applying the migrations")?;
    tracing::info!("migrations applied");
    Ok(())
}
