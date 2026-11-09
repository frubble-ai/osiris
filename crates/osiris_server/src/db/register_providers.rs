//! Each provider osiris implements is a row of `provider`, which connections and auth configs
//! reference. The rows are written from the registered providers whenever the migrations run, so
//! adding a provider needs no migration of its own.

use anyhow::Context;
use sqlx::postgres::PgConnection;

/// Adds a row for each of `ids` that has none yet.
pub async fn register_providers(connection: &mut PgConnection, ids: &[&str]) -> anyhow::Result<()> {
    let ids: Vec<String> = ids.iter().map(|id| (*id).to_owned()).collect();
    sqlx::query!(
        "INSERT INTO provider (key) SELECT unnest($1::text[]) ON CONFLICT (key) DO NOTHING",
        &ids
    )
    .execute(&mut *connection)
    .await
    .context("registering the providers")?;
    Ok(())
}
