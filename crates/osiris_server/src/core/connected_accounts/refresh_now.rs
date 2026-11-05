//! An operator asks for a connection to be refreshed now. A task runs the refresh immediately and
//! replaces the connection's scheduled refresh task, so the connection still has exactly one
//! pending refresh. Only an active connection with a refresh token can be refreshed.

use crate::core::{Core, CoreResult, refresh::scheduled::scheduled};
use osiris_common::errors::ReportSwitchExt;
use osiris_domain::{ConnectedAccount, DomainError};
use osiris_tasks::{RefreshReason, RefreshTask, Task, TaskPayload, names};
use time::OffsetDateTime;

impl Core {
    pub async fn refresh_now(&self, id: &str) -> CoreResult<ConnectedAccount> {
        let account = self.connected_account(id).await?;
        let refreshable = account
            .tokens
            .as_ref()
            .is_some_and(|tokens| tokens.refresh_token.is_some());
        if !account.is_complete() || !refreshable {
            return Err(DomainError::Conflict(
                "only a complete connection with a refresh token can be refreshed".into(),
            )
            .into());
        }
        let task = Task {
            name: names::refresh_now(&account.id, account.token_generation),
            run_at: OffsetDateTime::now_utc(),
            payload: TaskPayload::ConnectionRefresh(RefreshTask {
                tenant_id: account.user_id.clone(),
                connected_account_id: account.id.clone(),
                generation: account.token_generation,
                reason: RefreshReason::Retry,
            }),
        };
        self.schedule(&task).await?;
        let set = self
            .store
            .set_refresh_task(&account.id, account.token_generation, &scheduled(&task))
            .await
            .switch()?;
        if !set {
            // The connection changed meanwhile; the new task would do nothing anyway.
            self.cancel_quietly(&task.name).await;
            return Err(DomainError::Conflict("the connection changed meanwhile".into()).into());
        }
        if let Some(previous) = account.refresh_task.as_deref()
            && previous != task.name
        {
            self.cancel_quietly(previous).await;
        }
        self.connected_account(id).await
    }
}
