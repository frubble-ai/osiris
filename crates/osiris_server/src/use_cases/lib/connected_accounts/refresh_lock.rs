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
    lock_for_refresh::RefreshFlight,
    store_tokens::{Stamped, TokenWrite, store_tokens},
};
use crate::use_cases::UseCaseResult;
use crate::use_cases::lib::database::errors::db;
use osiris_common::crypto::Cipher;
use osiris_domain::{ConnectedAccount, ConnectionStatus, Event, EventType, TokenSet};
use sqlx::{Postgres, Transaction};

pub struct RefreshLock {
    tx: Transaction<'static, Postgres>,
    cipher: Cipher,
    pub account: ConnectedAccount,
    /// Held for the lock's life so a second refresh of this connection waits in-process, without a
    /// pool connection, until this one commits or rolls back.
    _flight: RefreshFlight,
}

/// The connection as the write left it, so nothing is read back after the commit.
fn written(
    mut account: ConnectedAccount,
    tokens: &NewTokens,
    status: ConnectionStatus,
    refresh: Option<&ScheduledRefresh>,
    stamped: &Stamped,
) -> ConnectedAccount {
    let held = account.tokens.take();
    account.tokens = Some(TokenSet {
        access_token: tokens.access_token.clone(),
        refresh_token: tokens
            .refresh_token
            .clone()
            .or_else(|| held.as_ref().and_then(|held| held.refresh_token.clone())),
        access_token_expires_at: tokens.access_token_expires_at,
        refresh_token_issued_at: stamped.refresh_token_issued_at,
        scope: tokens
            .scope
            .clone()
            .or_else(|| held.and_then(|held| held.scope)),
    });
    account.status = status;
    account.token_generation += 1;
    account.refresh_task = refresh.map(|refresh| refresh.name.clone());
    account.refresh_task_at = refresh.map(|refresh| refresh.at);
    account.last_refresh_error = None;
    account.refresh_failures = 0;
    account.updated_at = stamped.updated_at;
    account
}

impl RefreshLock {
    pub(crate) fn new(
        tx: Transaction<'static, Postgres>,
        cipher: Cipher,
        account: ConnectedAccount,
        flight: RefreshFlight,
    ) -> Self {
        Self {
            tx,
            cipher,
            account,
            _flight: flight,
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
        let stamped = store_tokens(
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
        Ok(written(
            self.account,
            tokens,
            ConnectionStatus::Active,
            refresh,
            &stamped,
        ))
    }

    /// The refresh worked but its next task could not be scheduled: saves the new tokens and
    /// disables the connection, with no task, until the refresh is scheduled again.
    pub async fn refreshed_unscheduled(
        mut self,
        tokens: &NewTokens,
        reason: &str,
    ) -> UseCaseResult<ConnectedAccount> {
        let id = self.account.id.clone();
        let stamped = store_tokens(
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
        let mut account = written(
            self.account,
            tokens,
            ConnectionStatus::Disabled,
            None,
            &stamped,
        );
        account.last_refresh_error = Some(reason.to_owned());
        Ok(account)
    }

    /// The provider permanently refused the refresh: the connection needs reconnecting.
    pub async fn expire(self, reason: &str) -> UseCaseResult<(Event, ConnectedAccount)> {
        self.end(reason, EventType::ConnectionExpired).await
    }

    /// The provider no longer has the grant: the connection needs reconnecting.
    pub async fn revoke(self, reason: &str) -> UseCaseResult<(Event, ConnectedAccount)> {
        self.end(reason, EventType::ConnectionRevoked).await
    }

    async fn end(
        mut self,
        reason: &str,
        event_type: EventType,
    ) -> UseCaseResult<(Event, ConnectedAccount)> {
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
        let mut account = self.account;
        account.status = ConnectionStatus::Expired;
        account.refresh_task = None;
        account.refresh_task_at = None;
        account.last_refresh_error = Some(reason.to_owned());
        Ok((Event::new(event_type, &id), account))
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
