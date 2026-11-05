//! A connection locked while it is refreshed, and the ways a refresh can end:
//!
//! - refreshed: the new tokens and their next refresh task are saved in one statement;
//! - refreshed but unscheduled: the next task could not be scheduled, so the new tokens are saved
//!   (a provider may already have invalidated the old refresh token) and the connection is
//!   disabled until its refresh is scheduled again;
//! - ended for good (expired or revoked): the connection needs reconnecting and nothing is
//!   scheduled;
//! - failed for now: the failure is counted, and the caller tries again later;
//! - released: nothing changed.
//!
//! Each of these ends the lock's transaction.

use super::{
    NewTokens, ScheduledRefresh,
    store_tokens::{TokenWrite, store_tokens},
};
use crate::use_cases::UseCaseResult;
use crate::use_cases::lib::database::Db;
use crate::use_cases::lib::database::errors::db;
use osiris_common::crypto::Cipher;
use osiris_domain::{ConnectedAccount, ConnectionStatus, Event, EventType};
use sqlx::{Postgres, Transaction};

pub struct RefreshLock {
    tx: Transaction<'static, Postgres>,
    db: Db,
    cipher: Cipher,
    pub account: ConnectedAccount,
}

impl RefreshLock {
    pub(crate) fn new(
        tx: Transaction<'static, Postgres>,
        db: Db,
        cipher: Cipher,
        account: ConnectedAccount,
    ) -> Self {
        Self {
            tx,
            db,
            cipher,
            account,
        }
    }

    /// The refresh worked: saves the new tokens, the next generation number and the next refresh
    /// task together.
    pub async fn refreshed(
        mut self,
        tokens: &NewTokens,
        refresh: Option<&ScheduledRefresh>,
    ) -> UseCaseResult<ConnectedAccount> {
        let id = self.account.id.clone();
        store_tokens(
            &mut self.tx,
            &self.cipher,
            TokenWrite {
                id: &id,
                tokens,
                generation: self.account.token_generation + 1,
                refresh,
                status: ConnectionStatus::Active,
                from: None,
            },
        )
        .await?;
        self.tx.commit().await.map_err(db)?;
        self.db.connected_account(&id).await
    }

    /// The refresh worked but its next task could not be scheduled: saves the new tokens and
    /// disables the connection, with no task, until the refresh is scheduled again.
    pub async fn refreshed_unscheduled(
        mut self,
        tokens: &NewTokens,
        reason: &str,
    ) -> UseCaseResult<ConnectedAccount> {
        let id = self.account.id.clone();
        store_tokens(
            &mut self.tx,
            &self.cipher,
            TokenWrite {
                id: &id,
                tokens,
                generation: self.account.token_generation + 1,
                refresh: None,
                status: ConnectionStatus::Disabled,
                from: None,
            },
        )
        .await?;
        sqlx::query!(
            "UPDATE connected_account SET last_refresh_error = $2 WHERE id = $1",
            id,
            reason
        )
        .execute(&mut *self.tx)
        .await
        .map_err(db)?;
        self.tx.commit().await.map_err(db)?;
        self.db.connected_account(&id).await
    }

    /// The provider permanently refused the refresh: the connection needs reconnecting.
    pub async fn expire(self, reason: &str) -> UseCaseResult<Event> {
        self.end(reason, EventType::ConnectionExpired).await
    }

    /// The provider no longer has the grant: the connection needs reconnecting.
    pub async fn revoke(self, reason: &str) -> UseCaseResult<Event> {
        self.end(reason, EventType::ConnectionRevoked).await
    }

    async fn end(mut self, reason: &str, event_type: EventType) -> UseCaseResult<Event> {
        let id = self.account.id.clone();
        sqlx::query!(
            "UPDATE connected_account SET status = 'expired', refresh_task = NULL, refresh_task_at = NULL,
                 last_refresh_error = $2, updated_at = now()
             WHERE id = $1",
            id,
            reason
        )
        .execute(&mut *self.tx)
        .await
        .map_err(db)?;
        self.tx.commit().await.map_err(db)?;
        Ok(Event::new(event_type, &id))
    }

    /// The refresh failed for now: saves the error and counts it. The tokens and their refresh task
    /// are left as they are.
    pub async fn failed(mut self, reason: &str) -> UseCaseResult<()> {
        let id = self.account.id.clone();
        sqlx::query!("UPDATE connected_account SET last_refresh_error = $2, refresh_failures = refresh_failures + 1 WHERE id = $1", id, reason)
            .execute(&mut *self.tx)
            .await
            .map_err(db)?;
        self.tx.commit().await.map_err(db)?;
        Ok(())
    }

    /// There was nothing to do: releases the lock without changes.
    pub async fn release(self) -> UseCaseResult<()> {
        self.tx.commit().await.map_err(db)?;
        Ok(())
    }
}
