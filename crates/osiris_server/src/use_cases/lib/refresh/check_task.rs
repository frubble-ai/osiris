//! What a refresh task finds when it locks its connection. A connection disabled because its last
//! refresh could not schedule the next one gets its refresh scheduled again, whichever refresh task
//! arrives. A connection that is no longer active, or is on another token generation, makes the task
//! out of date. Otherwise the refresh goes on.

use super::{Refresh, RunningTask, restore::restore};
use crate::{
    config::settings::refresh::RefreshSettings,
    use_cases::lib::{shared::domain_errors::UseCaseResult, tasks::Scheduling},
};
use osiris_common::errors::ReportSwitchExt;
use osiris_domain::ConnectionStatus;
use osiris_storage::{Store, connected_accounts::RefreshLock};

pub enum Checked {
    /// Go on with the refresh, still holding the lock.
    Go(Box<RefreshLock>),
    /// The task's work ends here; the lock is released.
    Done(Box<Refresh>),
}

pub async fn check_task(
    store: &Store,
    scheduling: Scheduling<'_>,
    refresh: &RefreshSettings,
    lock: RefreshLock,
    running: Option<RunningTask<'_>>,
) -> UseCaseResult<Checked> {
    let Some(task) = running else {
        return Ok(Checked::Go(Box::new(lock)));
    };
    let account = lock.account.clone();
    if account.status == ConnectionStatus::Disabled
        && account.tokens.is_some()
        && account.refresh_task.is_none()
    {
        lock.release().await.switch()?;
        return Ok(Checked::Done(Box::new(
            match restore(store, scheduling, refresh, &account).await? {
                Some(restored) => Refresh::Restored(restored),
                None => Refresh::Stale,
            },
        )));
    }
    if account.status != ConnectionStatus::Active || account.token_generation != task.generation {
        lock.release().await.switch()?;
        return Ok(Checked::Done(Box::new(Refresh::Stale)));
    }
    Ok(Checked::Go(Box::new(lock)))
}
