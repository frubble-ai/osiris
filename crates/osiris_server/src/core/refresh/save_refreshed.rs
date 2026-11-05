//! The provider answered a refresh with new tokens: schedules the next refresh task first, then
//! saves the new tokens and that task's name in one write, and cancels the previous task unless it
//! is the one running. If the next task cannot be scheduled, the new tokens are still saved (the
//! provider may already have invalidated the old refresh token) and the connection is disabled
//! until its refresh is scheduled again; the previous task is kept, so it can do that.

use super::{Refresh, RunningTask, scheduled::scheduled};
use crate::core::{Core, CoreResult};
use osiris_common::errors::ReportSwitchExt;
use osiris_storage::connected_accounts::{NewTokens, RefreshLock};

impl Core {
    pub(crate) async fn save_refreshed(
        &self,
        lock: RefreshLock,
        tokens: NewTokens,
        running: Option<RunningTask<'_>>,
    ) -> CoreResult<Refresh> {
        let account = lock.account.clone();
        let next = self.next_refresh(
            &account,
            account.token_generation + 1,
            tokens.access_token_expires_at,
        );
        if let Err(report) = self.schedule(&next).await {
            lock.refreshed_unscheduled(
                &tokens,
                "osiris could not schedule the next refresh; retrying",
            )
            .await
            .switch()?;
            return Err(report);
        }
        let refreshed = lock
            .refreshed(&tokens, Some(&scheduled(&next)))
            .await
            .switch()?;
        tracing::info!(connected_account_id = %account.id, next_refresh = %next.run_at, "tokens refreshed");
        self.cancel_previous(account.refresh_task.as_deref(), running)
            .await;
        Ok(Refresh::Refreshed(refreshed))
    }
}
