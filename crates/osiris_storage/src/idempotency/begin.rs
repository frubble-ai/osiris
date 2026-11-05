//! A request with an idempotency key arrives: decides whether to run it, replay the first answer,
//! or refuse it. A key older than `retention` is treated as unused and reused. A few expired keys
//! are also deleted each time.

use crate::{
    Store,
    errors::{StorageResult, db},
};
use osiris_domain::Begun;
use serde_json::Value;

impl Store {
    pub async fn begin_idempotent(
        &self,
        key: &str,
        request_hash: &str,
        retention: time::Duration,
        purge_batch: i64,
    ) -> StorageResult<Begun> {
        let retention = retention.as_seconds_f64();
        sqlx::query!(
            "DELETE FROM idempotency_key WHERE key IN (
                 SELECT key FROM idempotency_key WHERE created_at < now() - make_interval(secs => $1) AND key <> $2 LIMIT $3)",
            retention,
            key,
            purge_batch,
        )
        .execute(&self.pool)
        .await
        .map_err(db)?;
        let inserted = sqlx::query!(
            "INSERT INTO idempotency_key (key, request_hash) VALUES ($1, $2)
             ON CONFLICT (key) DO UPDATE SET request_hash = EXCLUDED.request_hash, status = NULL, body = NULL, created_at = now()
             WHERE idempotency_key.created_at < now() - make_interval(secs => $3)",
            key,
            request_hash,
            retention,
        )
        .execute(&self.pool)
        .await
        .map_err(db)?
        .rows_affected();
        if inserted == 1 {
            return Ok(Begun::New);
        }
        let held = sqlx::query!(
            "SELECT request_hash, status, body FROM idempotency_key WHERE key = $1",
            key
        )
        .fetch_one(&self.pool)
        .await
        .map_err(db)?;
        if held.request_hash != request_hash {
            return Ok(Begun::Mismatch);
        }
        Ok(match held.status {
            Some(status) => Begun::Replay {
                status: u16::try_from(status).unwrap_or(500),
                body: held.body.unwrap_or(Value::Null),
            },
            None => Begun::InFlight,
        })
    }
}
