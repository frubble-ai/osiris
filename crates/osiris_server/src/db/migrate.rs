use super::{apply, connect, drop_owned};
use crate::config::MigrateConfig;
use anyhow::Context;

/// Apply every pending migration. With `config.reset`, drop what this user owns in `public`
/// first, so the migrations run on an empty database. Already applied ones are skipped when
/// not resetting.
pub async fn migrate(config: &MigrateConfig) -> anyhow::Result<()> {
    let pool = connect(&config.database).await?;
    if config.reset {
        let connected = sqlx::query_scalar!(r#"SELECT current_database()::text AS "name!" "#)
            .fetch_one(&pool)
            .await
            .context("reading the connected database")?;
        crate::config::migrate::require_reset_target(
            true,
            config.reset_database.as_deref(),
            &connected,
        )?;
        drop_owned(&pool).await?;
    }
    apply::apply(&pool).await
}
