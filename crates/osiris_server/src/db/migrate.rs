use super::{apply, drop_owned, exclude::MigrateExclusion};
use crate::config::MigrateConfig;
use anyhow::Context;

/// Apply every pending migration. With `config.reset`, drop what this user owns in `public`
/// first, so the migrations run on an empty database. Already applied ones are skipped when
/// not resetting.
///
/// The exclusion is acquired before anything is dropped or applied, and the drop and the
/// migrations run on the connection that holds it. A second execution blocks on the lock
/// instead of changing the schema at the same time. If that session closes, the work stops
/// with the lock.
pub async fn migrate(config: &MigrateConfig) -> anyhow::Result<()> {
    let mut exclusion = MigrateExclusion::acquire(&config.database).await?;
    if config.reset {
        let connected = sqlx::query_scalar!(r#"SELECT current_database()::text AS "name!" "#)
            .fetch_one(&mut *exclusion.connection())
            .await
            .context("reading the connected database")?;
        crate::config::migrate::require_reset_target(
            true,
            config.reset_database.as_deref(),
            &connected,
        )?;
        drop_owned(&mut *exclusion.connection()).await?;
    }
    apply::apply(exclusion.connection()).await
}
