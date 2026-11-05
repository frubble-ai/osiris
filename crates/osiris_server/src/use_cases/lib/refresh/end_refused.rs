//! The provider permanently refused a refresh: the connection is marked as needing reconnection
//! with a fixed reason, `connection.expired` is published, and its previous refresh task is
//! cancelled unless it is the one running. Nothing is scheduled.

use super::{Refresh, RunningTask, cancel_previous::cancel_previous};
use crate::use_cases::lib::{
    events::publish::publish, shared::domain_errors::UseCaseResult, tasks::Scheduling,
};
use osiris_common::errors::ReportSwitchExt;
use osiris_events::MessageBus;
use osiris_storage::{Store, connected_accounts::RefreshLock};

pub async fn end_refused(
    store: &Store,
    messages: &MessageBus,
    scheduling: Scheduling<'_>,
    lock: RefreshLock,
    previous_task: Option<&str>,
    running: Option<RunningTask<'_>>,
) -> UseCaseResult<Refresh> {
    let id = lock.account.id.clone();
    let event = lock
        .expire("the provider refused the refresh; reconnect")
        .await
        .switch()?;
    publish(messages, scheduling, &event).await;
    cancel_previous(scheduling, previous_task, running).await;
    Ok(Refresh::Ended(store.connected_account(&id).await.switch()?))
}
