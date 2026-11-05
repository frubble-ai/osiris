//! The provider answered a refresh with new tokens: schedules the next refresh task first, then
//! saves the new tokens and that task's name in one write, and cancels the previous task unless it
//! is the one running. If the next task cannot be scheduled, the new tokens are still saved (the
//! provider may already have invalidated the old refresh token) and the connection is disabled
//! until its refresh is scheduled again; the previous task is kept, so it can do that.
//!
//! The provider has already rotated the tokens by now, so this runs as its own task: a request
//! that times out or a client that disconnects can't drop it before the new tokens are saved.

use super::{
    Refresh, RunningTask, cancel_previous::cancel_previous, next_refresh::next_refresh,
    scheduled::scheduled,
};
use crate::use_cases::lib::connected_accounts::{NewTokens, RefreshLock};
use crate::{
    config::settings::refresh::RefreshSettings,
    use_cases::lib::{
        shared::domain_errors::{UseCaseResult, internal},
        tasks::{Scheduling, schedule::schedule},
    },
};
use osiris_tasks::TaskScheduler;
use std::{sync::Arc, time::Duration};

/// The parts the save needs, owned, since it runs on its own.
pub struct SaveParts {
    pub tasks: Arc<dyn TaskScheduler>,
    pub schedule_timeout: Duration,
    pub refresh: RefreshSettings,
}

pub async fn save_refreshed(
    parts: SaveParts,
    lock: RefreshLock,
    tokens: NewTokens,
    running: Option<RunningTask<'_>>,
) -> UseCaseResult<Refresh> {
    let running = running.map(|task| (task.name.to_owned(), task.generation));
    tokio::spawn(async move {
        let running = running.as_ref().map(|(name, generation)| RunningTask {
            name,
            generation: *generation,
        });
        save_now(&parts, lock, tokens, running).await
    })
    .await
    .map_err(internal)?
}

async fn save_now(
    parts: &SaveParts,
    lock: RefreshLock,
    tokens: NewTokens,
    running: Option<RunningTask<'_>>,
) -> UseCaseResult<Refresh> {
    let scheduling = Scheduling {
        tasks: parts.tasks.as_ref(),
        timeout: parts.schedule_timeout,
    };
    let account = lock.account.clone();
    let next = next_refresh(
        &parts.refresh,
        &account,
        account.token_generation + 1,
        tokens.access_token_expires_at,
    );
    if let Err(report) = schedule(scheduling, &next).await {
        lock.refreshed_unscheduled(
            &tokens,
            "osiris could not schedule the next refresh; retrying",
        )
        .await?;
        return Err(report);
    }
    let refreshed = lock.refreshed(&tokens, Some(&scheduled(&next))).await?;
    tracing::info!(connected_account_id = %account.id, next_refresh = %next.run_at, "tokens refreshed");
    cancel_previous(scheduling, account.refresh_task.as_deref(), running).await;
    Ok(Refresh::Refreshed(refreshed))
}
