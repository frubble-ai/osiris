//! Schedules the refresh of a disabled connection that holds tokens but no refresh task (its
//! last refresh could not schedule the next one), and makes it active again. Returns `None` when
//! the connection is not in that state, so there is nothing to restore.

use super::scheduled::scheduled;
use crate::core::{Core, CoreResult};
use osiris_common::errors::ReportSwitchExt;
use osiris_domain::{ConnectedAccount, ConnectionStatus};

impl Core {
    pub(crate) async fn restore(
        &self,
        account: &ConnectedAccount,
    ) -> CoreResult<Option<ConnectedAccount>> {
        let Some(tokens) = account.tokens.as_ref() else {
            return Ok(None);
        };
        if account.status != ConnectionStatus::Disabled || account.refresh_task.is_some() {
            return Ok(None);
        }
        let task = self.next_refresh(
            account,
            account.token_generation,
            tokens.access_token_expires_at,
        );
        self.schedule(&task).await?;
        // The task's name comes from the connection and generation, so a concurrent restore
        // schedules this same task: whichever restore saves first, the task is the connection's,
        // and it is never cancelled here.
        let restored = self
            .store
            .restore(&account.id, account.token_generation, &scheduled(&task))
            .await
            .switch()?;
        tracing::info!(connected_account_id = %account.id, restored, "refresh scheduled; the connection is active");
        Ok(Some(
            self.store.connected_account(&account.id).await.switch()?,
        ))
    }
}
