//! The reset drops tables and enums this database user owns, including what depends on them, and
//! leaves a relation someone else owns.

#![allow(
    clippy::unwrap_used,
    reason = "tests: a failure here is the test failing"
)]

use super::{MIGRATOR, drop_owned};
use crate::config::{DatabaseConfig, MigrateConfig, Settings};
use crate::db::migrate;
use sqlx::PgPool;

/// Drops `role` when the test ends, including when an assertion fails. `#[sqlx::test]`
/// drops only the per-test database, so a role would otherwise stay on the shared server.
struct RoleGuard {
    pool: PgPool,
    role: String,
}

impl RoleGuard {
    async fn create(pool: &PgPool, role: String) -> Self {
        drop_role(pool, &role).await;
        let sql = format!("CREATE ROLE {role}");
        sqlx::query(sqlx::AssertSqlSafe(sql))
            .execute(pool)
            .await
            .unwrap();
        Self {
            pool: pool.clone(),
            role,
        }
    }
}

impl Drop for RoleGuard {
    fn drop(&mut self) {
        let pool = self.pool.clone();
        let role = self.role.clone();
        let _ = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            tokio::task::block_in_place(|| {
                tokio::runtime::Handle::current().block_on(async move {
                    drop_role(&pool, &role).await;
                });
            });
        }));
    }
}

async fn drop_role(pool: &PgPool, role: &str) {
    // The role may own the kept table when an assertion fails first. DROP OWNED
    // removes those in this database; it errors when the role is already gone.
    let owned = format!("DROP OWNED BY {role}");
    let _ = sqlx::query(sqlx::AssertSqlSafe(owned)).execute(pool).await;
    let sql = format!("DROP ROLE IF EXISTS {role}");
    let _ = sqlx::query(sqlx::AssertSqlSafe(sql)).execute(pool).await;
}

#[sqlx::test(migrator = "MIGRATOR")]
async fn reset_drops_tables_and_enums_this_user_owns(pool: PgPool) {
    let providers = sqlx::query_scalar!(r#"SELECT count(*) AS "count!" FROM provider"#)
        .fetch_one(&pool)
        .await
        .unwrap();
    assert_eq!(providers, 3, "the migrator has applied the migrations");

    // The role name has to be an identifier, which a bound parameter cannot be. It is the test
    // database's name, which is unique to this run and contains only letters, digits and '_'.
    let database = sqlx::query_scalar!(r#"SELECT current_database()::text AS "name!" "#)
        .fetch_one(&pool)
        .await
        .unwrap();
    let role: String = database
        .chars()
        .map(|character| {
            if character.is_ascii_alphanumeric() {
                character
            } else {
                '_'
            }
        })
        .take(63)
        .collect();
    assert!(
        role.chars()
            .all(|character| character.is_ascii_alphanumeric() || character == '_')
            && role.starts_with(|character: char| {
                character.is_ascii_alphabetic() || character == '_'
            }),
        "{role}"
    );

    // CREATE TYPE and ALTER OWNER are DDL, so query! cannot prepare them.
    // The role name is an identifier, so it cannot be a bind parameter either.
    // The guard drops the role on every exit, including a failed assertion.
    let _role = RoleGuard::create(&pool, role.clone()).await;
    execute(&pool, "CREATE TYPE public.reset_probe AS ENUM ('a')").await;
    execute(
        &pool,
        "CREATE TABLE public.reset_owned (id int PRIMARY KEY)",
    )
    .await;
    execute(
        &pool,
        "CREATE VIEW public.reset_owned_view AS SELECT id FROM public.reset_owned",
    )
    .await;
    execute(
        &pool,
        "CREATE TABLE public.reset_kept (
            id int PRIMARY KEY,
            owned_id int CONSTRAINT reset_kept_owned_fk REFERENCES public.reset_owned (id)
        )",
    )
    .await;
    execute_owned(
        &pool,
        format!("ALTER TABLE public.reset_kept OWNER TO {role}"),
    )
    .await;
    execute(&pool, "CREATE TYPE public.reset_kept_enum AS ENUM ('b')").await;
    execute_owned(
        &pool,
        format!("ALTER TYPE public.reset_kept_enum OWNER TO {role}"),
    )
    .await;

    drop_owned(&pool).await.unwrap();

    let owned_tables = sqlx::query_scalar!(
        r#"SELECT count(*) AS "count!" FROM pg_tables WHERE schemaname = 'public' AND tableowner = current_user"#
    )
    .fetch_one(&pool)
    .await
    .unwrap();
    assert_eq!(owned_tables, 0);
    let owned_enums = sqlx::query_scalar!(
        r#"SELECT count(*) AS "count!"
           FROM pg_type type
           JOIN pg_namespace namespace ON namespace.oid = type.typnamespace
           WHERE namespace.nspname = 'public'
             AND type.typtype = 'e'
             AND pg_get_userbyid(type.typowner) = current_user"#
    )
    .fetch_one(&pool)
    .await
    .unwrap();
    assert_eq!(owned_enums, 0);
    let kept_table = sqlx::query_scalar!(
        r#"SELECT count(*) AS "count!" FROM pg_tables WHERE schemaname = 'public' AND tablename = 'reset_kept'"#
    )
    .fetch_one(&pool)
    .await
    .unwrap();
    assert_eq!(kept_table, 1);
    let kept_enum = sqlx::query_scalar!(
        r#"SELECT count(*) AS "count!"
           FROM pg_type type
           JOIN pg_namespace namespace ON namespace.oid = type.typnamespace
           WHERE namespace.nspname = 'public' AND type.typname = 'reset_kept_enum'"#
    )
    .fetch_one(&pool)
    .await
    .unwrap();
    assert_eq!(kept_enum, 1);
    // CASCADE removes the view that depended on the dropped table, and the foreign key on the
    // table that was kept.
    let view = sqlx::query_scalar!(
        r#"SELECT count(*) AS "count!" FROM pg_views WHERE schemaname = 'public' AND viewname = 'reset_owned_view'"#
    )
    .fetch_one(&pool)
    .await
    .unwrap();
    assert_eq!(view, 0);
    let foreign_key = sqlx::query_scalar!(
        r#"SELECT count(*) AS "count!" FROM pg_constraint WHERE conname = 'reset_kept_owned_fk'"#
    )
    .fetch_one(&pool)
    .await
    .unwrap();
    assert_eq!(foreign_key, 0);

    MIGRATOR.run(&pool).await.unwrap();
    let providers = sqlx::query_scalar!(r#"SELECT count(*) AS "count!" FROM provider"#)
        .fetch_one(&pool)
        .await
        .unwrap();
    assert_eq!(
        providers, 3,
        "the migrations run again on the empty database"
    );

    execute(&pool, "DROP TABLE public.reset_kept").await;
    execute(&pool, "DROP TYPE public.reset_kept_enum").await;
}

#[sqlx::test(migrator = "MIGRATOR")]
async fn migrate_drops_only_when_reset_and_drops_before_applying(pool: PgPool) {
    let name = sqlx::query_scalar!(r#"SELECT current_database()::text AS "name!" "#)
        .fetch_one(&pool)
        .await
        .unwrap();
    execute(
        &pool,
        "CREATE TABLE public.reset_marker (id int PRIMARY KEY)",
    )
    .await;

    migrate(&migrate_config(&name, false)).await.unwrap();
    assert_eq!(marker_tables(&pool).await, 1, "a migrate leaves the marker");

    migrate(&migrate_config(&name, true)).await.unwrap();
    assert_eq!(
        marker_tables(&pool).await,
        0,
        "a reset drops the marker before the migrations run again"
    );
    let providers = sqlx::query_scalar!(r#"SELECT count(*) AS "count!" FROM provider"#)
        .fetch_one(&pool)
        .await
        .unwrap();
    assert_eq!(providers, 3, "the migrations ran again after the drop");
}

fn migrate_config(database_name: &str, reset: bool) -> MigrateConfig {
    let database = DatabaseConfig::from_vars(
        &|key| match key {
            "DB_NAME" => Some(database_name.to_owned()),
            other => std::env::var(other).ok(),
        },
        &Settings::default(),
    )
    .unwrap();
    MigrateConfig {
        database,
        reset,
        reset_database: reset.then(|| database_name.to_owned()),
    }
}

async fn marker_tables(pool: &PgPool) -> i64 {
    sqlx::query_scalar!(
        r#"SELECT count(*) AS "count!" FROM pg_tables
           WHERE schemaname = 'public' AND tablename = 'reset_marker'"#
    )
    .fetch_one(pool)
    .await
    .unwrap()
}

async fn execute(pool: &PgPool, sql: &'static str) {
    sqlx::query(sql).execute(pool).await.unwrap();
}

/// `sql` embeds `role`, which is the test database name reduced to letters, digits and `_`.
async fn execute_owned(pool: &PgPool, sql: String) {
    sqlx::query(sqlx::AssertSqlSafe(sql))
        .execute(pool)
        .await
        .unwrap();
}
