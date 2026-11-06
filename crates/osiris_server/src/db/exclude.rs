//! One migrate execution at a time, held by the process rather than by the workflow.
//!
//! GitHub kills a cancelled job's cleanup step five minutes after the cancellation, which
//! lets the next deploy or reset start while this execution is still running. A session
//! advisory lock on a connection this process keeps open until it exits does not depend on
//! that runner: the next execution waits here before it drops or migrates, and the lock
//! goes when this process does.
//!
//! sqlx takes its own advisory lock, one bigint derived from the database name, on whatever
//! connection applies the migrations, and releases it when that is done. This one uses the
//! two-integer form, which is a different lock, and it is held on a connection that does
//! nothing else, so the two cannot block each other.

use crate::config::DatabaseConfig;
use anyhow::Context;
use sqlx::Connection;
use sqlx::postgres::PgConnection;

/// First key of the two-integer advisory lock. Chosen so it is not sqlx's lock.
pub(crate) const LOCK_CLASS: i32 = 0x0515_1501;
/// Second key. The pair is the whole lock; the value only has to stay stable.
pub(crate) const LOCK_OBJECT: i32 = 1;

/// The open connection that holds the lock. Dropping it closes the session, which
/// releases the lock, including when the migration fails.
pub(crate) struct MigrateExclusion {
    /// Kept so the session, and the lock, stay open until this value is dropped.
    #[allow(dead_code, reason = "holding the connection is what holds the lock")]
    connection: PgConnection,
}

impl MigrateExclusion {
    /// Connects and waits until no other migrate execution holds the lock.
    pub(crate) async fn acquire(config: &DatabaseConfig) -> anyhow::Result<Self> {
        let mut connection = tokio::time::timeout(
            config.connect_timeout,
            PgConnection::connect_with(&config.connect_options()),
        )
        .await
        .context("timed out connecting to hold the migrate lock")?
        .context("connecting to hold the migrate lock")?;

        // A server `lock_timeout` or `statement_timeout` would abort the wait and let this
        // execution proceed while the other is still changing the schema. This session
        // waits until the lock is free. `set_config` and `pg_advisory_lock` are `sqlx::query`:
        // the first returns text we do not read, and the second returns void, which `query!`
        // cannot describe.
        sqlx::query(
            "SELECT set_config('lock_timeout', '0', false), \
             set_config('statement_timeout', '0', false)",
        )
        .execute(&mut connection)
        .await
        .context("disabling the lock and statement timeouts on the migrate lock")?;
        tracing::info!("taking the migrate lock");
        sqlx::query("SELECT pg_advisory_lock($1, $2)")
            .bind(LOCK_CLASS)
            .bind(LOCK_OBJECT)
            .execute(&mut connection)
            .await
            .context("waiting for the other migrate execution to finish")?;
        Ok(Self { connection })
    }
}
