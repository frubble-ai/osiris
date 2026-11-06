//! One migrate execution at a time, held by the process rather than by the workflow.
//!
//! GitHub kills a cancelled job's cleanup step five minutes after the cancellation, which
//! lets the next deploy or reset start while this execution is still running. A session
//! advisory lock on a connection this process keeps open until it exits does not depend on
//! that runner: the next execution waits here before it drops or migrates, and the lock
//! goes when this process does.
//!
//! The drop and the migrations run on this same connection. If PostgreSQL closes the session,
//! the lock is released and that work stops with it; nothing carries on through another
//! connection. sqlx takes its own advisory lock on this connection while it applies the
//! migrations and releases that lock when it finishes. This one uses the two-integer form,
//! a different lock, and stays until the connection closes.

use crate::config::DatabaseConfig;
use anyhow::Context;
use sqlx::Connection;
use sqlx::postgres::PgConnection;

/// First key of the two-integer advisory lock. Chosen so it is not sqlx's lock.
pub(crate) const LOCK_CLASS: i32 = 0x0515_1501;
/// Second key. The pair is the whole lock; the value only has to stay stable.
pub(crate) const LOCK_OBJECT: i32 = 1;

/// The open connection that holds the lock. The drop and the migrations run on it.
/// Dropping it closes the session, which releases the lock and stops that work.
pub(crate) struct MigrateExclusion {
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
        // waits until the lock is free.
        sqlx::query!(
            "SELECT set_config('lock_timeout', '0', false) AS lock_timeout, \
                    set_config('statement_timeout', '0', false) AS statement_timeout"
        )
        .fetch_one(&mut connection)
        .await
        .context("disabling the lock and statement timeouts on the migrate lock")?;
        tracing::info!("taking the migrate lock");
        // `pg_advisory_lock` returns void. sqlx describes that as an empty row, so `query!`
        // checks it and `execute` does not read a value.
        sqlx::query!("SELECT pg_advisory_lock($1, $2)", LOCK_CLASS, LOCK_OBJECT)
            .execute(&mut connection)
            .await
            .context("waiting for the other migrate execution to finish")?;
        Ok(Self { connection })
    }

    /// The session that holds the lock. Schema changes go through here, so they cannot
    /// outlive the lock.
    pub(crate) fn connection(&mut self) -> &mut PgConnection {
        &mut self.connection
    }
}
