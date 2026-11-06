//! The database connection, and the migration job. The pool opens its first connection at
//! startup, so a database that can't be reached, or whose TLS can't be established against
//! `DB_CA_PEM`, stops the process right away instead of failing the first request.

use crate::config::{DatabaseConfig, MigrateConfig};
use anyhow::Context;
use sqlx::postgres::{PgPool, PgPoolOptions};

/// The migrations in `migrations/`, compiled in.
pub static MIGRATOR: sqlx::migrate::Migrator = sqlx::migrate!("../../migrations");

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

/// Tables and enums this database user owns in `public`. A reset drops only those, so a
/// relation someone else created stays. `_sqlx_migrations` is one of the tables, and the
/// migrator then runs every migration again from nothing.
///
/// This is `sqlx::query`, not `query!`. `query!` prepares the statement so it can check it,
/// and PostgreSQL refuses `PREPARE` of a `DO` block. The block's `EXECUTE format(...)` is
/// dynamic SQL besides, so there is no statement sqlx can describe at compile time.
const DROP_OWNED_SQL: &str = r"
DO $$
DECLARE item record;
BEGIN
    FOR item IN
        SELECT tablename FROM pg_tables
        WHERE schemaname = 'public' AND tableowner = current_user
    LOOP
        EXECUTE format('DROP TABLE public.%I CASCADE', item.tablename);
    END LOOP;
    FOR item IN
        SELECT type.typname
        FROM pg_type type
        JOIN pg_namespace namespace ON namespace.oid = type.typnamespace
        WHERE namespace.nspname = 'public'
          AND type.typtype = 'e'
          AND pg_get_userbyid(type.typowner) = current_user
    LOOP
        EXECUTE format('DROP TYPE public.%I CASCADE', item.typname);
    END LOOP;
END
$$;
";

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
    apply(&pool).await
}

async fn drop_owned(pool: &PgPool) -> anyhow::Result<()> {
    tracing::warn!(
        "osiris migrate reset: dropping the tables and types this database user owns in public"
    );
    sqlx::query(DROP_OWNED_SQL)
        .execute(pool)
        .await
        .context("dropping the database's tables and types")?;
    Ok(())
}

async fn apply(pool: &PgPool) -> anyhow::Result<()> {
    MIGRATOR
        .run(pool)
        .await
        .context("applying the migrations")?;
    tracing::info!("migrations applied");
    Ok(())
}

#[cfg(test)]
mod tests;
