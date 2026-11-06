//! An operator asks for a connection to be refreshed now. A task runs the refresh immediately and
//! replaces the connection's scheduled refresh task, so the connection still has exactly one
//! pending refresh. Only an active connection with a refresh token can be refreshed.

use crate::use_cases::{
    UseCaseResult, UseCases,
    cases::get_connected_account::GetConnectedAccountArgs,
    lib::{
        refresh::scheduled::scheduled,
        tasks::{cancel_quietly::cancel_quietly, schedule::schedule},
    },
};
use osiris_domain::{ConnectedAccount, DomainError};
use osiris_tasks::{RefreshReason, RefreshTask, Task, TaskPayload, names};
use time::OffsetDateTime;

pub struct RefreshConnectedAccountNowArgs<'a> {
    pub id: &'a str,
}

impl UseCases {
    pub async fn refresh_connected_account_now(
        &self,
        args: RefreshConnectedAccountNowArgs<'_>,
    ) -> UseCaseResult<ConnectedAccount> {
        self.run("refresh_connected_account_now", async {
            let ctx = self.context();
            let account = self
                .get_connected_account(GetConnectedAccountArgs { id: args.id })
                .await?;
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
            schedule(ctx.scheduling(), &task).await?;
            let set = ctx
                .db
                .set_refresh_task(&account.id, account.token_generation, &scheduled(&task))
                .await?;
            if !set {
                // The connection changed meanwhile; the new task would do nothing anyway.
                cancel_quietly(ctx.scheduling(), &task.name).await;
                return Err(
                    DomainError::Conflict("the connection changed meanwhile".into()).into(),
                );
            }
            if let Some(previous) = account.refresh_task.as_deref()
                && previous != task.name
            {
                cancel_quietly(ctx.scheduling(), previous).await;
            }
            self.get_connected_account(GetConnectedAccountArgs { id: args.id })
                .await
        })
        .await
    }
}
