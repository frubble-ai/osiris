use super::{MIGRATOR, register_providers::register_providers};
use anyhow::Context;
use osiris_providers::REGISTRATIONS;
use sqlx::postgres::PgConnection;

pub(super) async fn apply(connection: &mut PgConnection) -> anyhow::Result<()> {
    MIGRATOR
        .run(&mut *connection)
        .await
        .context("applying the migrations")?;
    tracing::info!("migrations applied");
    let ids: Vec<&str> = REGISTRATIONS
        .iter()
        .map(|registration| registration.id)
        .collect();
    register_providers(connection, &ids).await
}
