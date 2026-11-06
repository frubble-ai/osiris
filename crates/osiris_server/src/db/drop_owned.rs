use anyhow::Context;
use sqlx::postgres::PgConnection;

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

pub(crate) async fn drop_owned(connection: &mut PgConnection) -> anyhow::Result<()> {
    tracing::warn!(
        "osiris migrate reset: dropping the tables and types this database user owns in public"
    );
    sqlx::query(DROP_OWNED_SQL)
        .execute(&mut *connection)
        .await
        .context("dropping the database's tables and types")?;
    Ok(())
}
