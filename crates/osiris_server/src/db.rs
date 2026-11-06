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

/// Tables and enums this database user owns in `public`. A reset drops only those, so a
/// relation someone else created stays. `_sqlx_migrations` is one of the tables, and the
/// migrator then runs every migration again from nothing.
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

/// `OSIRIS_ENV` values a `--reset` may run in. prd is refused here, in the process, so a
/// caller that can execute the job still cannot wipe it.
pub fn reset_allowed(environment: Option<&str>) -> bool {
    matches!(environment, Some("local" | "dev" | "stg"))
}

/// Apply every pending migration. With `reset`, drop what this user owns first, so the
/// migrations run on an empty database. Already applied ones are skipped when not resetting.
pub async fn migrate(config: &DatabaseConfig, reset: bool) -> anyhow::Result<()> {
    let pool = connect(config).await?;
    if reset {
        let environment = crate::config::env_vars::optional("OSIRIS_ENV");
        if !reset_allowed(environment.as_deref()) {
            anyhow::bail!(
                "--reset is refused with OSIRIS_ENV '{}': only local, dev and stg reset",
                environment.as_deref().unwrap_or("unset")
            );
        }
        tracing::warn!("dropping the tables and types this database user owns, then migrating");
        sqlx::query(DROP_OWNED_SQL)
            .execute(&pool)
            .await
            .context("dropping the database's tables and types")?;
    }
    MIGRATOR
        .run(&pool)
        .await
        .context("applying the migrations")?;
    tracing::info!("migrations applied");
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::reset_allowed;

    #[test]
    fn reset_is_only_local_dev_or_stg() {
        assert!(reset_allowed(Some("local")));
        assert!(reset_allowed(Some("dev")));
        assert!(reset_allowed(Some("stg")));
        assert!(!reset_allowed(Some("prd")));
        assert!(!reset_allowed(Some("STG")));
        assert!(!reset_allowed(Some("")));
        assert!(!reset_allowed(None));
    }
}
