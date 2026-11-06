use super::MIGRATOR;
use anyhow::Context;
use sqlx::postgres::PgConnection;

pub(super) async fn apply(connection: &mut PgConnection) -> anyhow::Result<()> {
    MIGRATOR
        .run(&mut *connection)
        .await
        .context("applying the migrations")?;
    tracing::info!("migrations applied");
    Ok(())
}
