//! Schedules the refresh of a disabled connection that holds tokens but no refresh task (its last
//! refresh could not schedule the next one), and makes it active again. Returns `None` when the
//! connection is not in that state, or another write got there first, so there is nothing to
//! restore.

use super::{next_refresh::next_refresh, scheduled::scheduled};
use crate::use_cases::lib::database::Db;
use crate::{
    config::settings::refresh::RefreshSettings,
    use_cases::lib::{
        shared::domain_errors::UseCaseResult,
        tasks::{Scheduling, schedule::schedule},
    },
};
use osiris_domain::{ConnectedAccount, ConnectionStatus};

pub async fn restore(
    db: &Db,
    scheduling: Scheduling<'_>,
    refresh: &RefreshSettings,
    account: &ConnectedAccount,
) -> UseCaseResult<Option<ConnectedAccount>> {
    let Some(tokens) = account.tokens.as_ref() else {
        return Ok(None);
    };
    if account.status != ConnectionStatus::Disabled || account.refresh_task.is_some() {
        return Ok(None);
    }
    let task = next_refresh(
        refresh,
        account,
        account.token_generation,
        tokens.access_token_expires_at,
    );
    schedule(scheduling, &task).await?;
    // The task's name comes from the connection and generation, so a concurrent restore schedules
    // this same task: whichever restore saves first, the task is the connection's, and it is never
    // cancelled here.
    let restored = db
        .restore(&account.id, account.token_generation, &scheduled(&task))
        .await?;
    tracing::info!(connected_account_id = %account.id, restored, "refresh scheduled; the connection is active");
    if !restored {
        return Ok(None);
    }
    let mut account = account.clone();
    account.status = ConnectionStatus::Active;
    account.refresh_task = Some(task.name.clone());
    account.refresh_task_at = Some(task.run_at);
    account.last_refresh_error = None;
    Ok(Some(account))
}
