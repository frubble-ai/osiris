//! Lock a connection for a refresh: concurrent refreshes of it wait here, then see the tokens the
//! first one stored.

use super::RefreshLock;
use crate::use_cases::UseCaseResult;
use crate::use_cases::lib::database::Db;
use crate::use_cases::lib::database::Tenant;
use crate::use_cases::lib::database::errors::db;
use crate::use_cases::lib::database::rows::AccountRow;
use osiris_domain::DomainError;
use std::sync::{Arc, Mutex, PoisonError};
use tokio::sync::{Mutex as AsyncMutex, OwnedMutexGuard};

/// An in-process flight for one connection: waiters block here before taking a pool connection.
/// Dropped when the refresh ends; removes its map entry when nothing else holds a clone.
pub(crate) struct RefreshFlight {
    id: String,
    flights: Arc<Mutex<std::collections::HashMap<String, Arc<AsyncMutex<()>>>>>,
    flight: Arc<AsyncMutex<()>>,
    guard: Option<OwnedMutexGuard<()>>,
}

impl RefreshFlight {
    async fn acquire(db: &Db, id: &str) -> Self {
        let flight = {
            let mut flights = db
                .refresh_flights
                .lock()
                .unwrap_or_else(PoisonError::into_inner);
            flights
                .entry(id.to_owned())
                .or_insert_with(|| Arc::new(AsyncMutex::new(())))
                .clone()
        };
        let guard = flight.clone().lock_owned().await;
        Self {
            id: id.to_owned(),
            flights: db.refresh_flights.clone(),
            flight,
            guard: Some(guard),
        }
    }
}

impl Drop for RefreshFlight {
    fn drop(&mut self) {
        self.guard.take();
        let mut flights = self.flights.lock().unwrap_or_else(PoisonError::into_inner);
        // map entry + this struct: nothing else is waiting or holding.
        if Arc::strong_count(&self.flight) == 2 {
            flights.remove(&self.id);
        }
    }
}

impl Db {
    pub async fn lock_for_refresh(&self, id: &str) -> UseCaseResult<RefreshLock> {
        // Wait in-process first so only one refresh of this connection holds a pool connection.
        let flight = RefreshFlight::acquire(self, id).await;

        let mut tx = self.pool.begin().await.map_err(db)?;
        // Cross-instance waiters still give up after `lock_timeout` instead of holding a connection
        // for the length of another instance's provider call.
        let milliseconds = u64::try_from(self.lock_timeout.as_millis()).unwrap_or(u64::MAX);
        sqlx::query_scalar!(
            "SELECT set_config('lock_timeout', $1, true)",
            format!("{milliseconds}ms")
        )
        .fetch_one(&mut *tx)
        .await
        .map_err(db)?;
        let row = sqlx::query_as!(AccountRow, "SELECT id, auth_config_id, provider, user_id, alias, status, credential_kind, installation_id, callback_url, access_token, refresh_token, access_token_expires_at, refresh_token_issued_at, scope, last_refresh_error, refresh_failures, token_generation, refresh_task, refresh_task_at, created_at, updated_at FROM connected_account WHERE id = $1 FOR UPDATE", id)
            .fetch_optional(&mut *tx)
            .await
            .map_err(db)?
            .ok_or(DomainError::NotFound("connected account"))?;
        let cipher = self.cipher_for(&Tenant::user(&row.user_id)).await?;
        let account = row.decrypt(&cipher)?;
        Ok(RefreshLock::new(tx, cipher, account, flight))
    }
}
