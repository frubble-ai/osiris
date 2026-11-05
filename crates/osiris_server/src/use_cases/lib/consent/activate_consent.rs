//! The code was exchanged for tokens: schedules the first refresh when the provider issues refresh
//! tokens, then saves the tokens and the task's name together, which makes the connection active.
//! A provider that should have sent a refresh token and didn't, or a first refresh that cannot be
//! scheduled, disables the connection without keeping the tokens.

use super::{
    Outcome,
    back_to::back_to,
    consent_failed::consent_failed,
    disable_consent::disable_consent,
    first_refresh::{FirstRefresh, first_refresh},
};
use crate::{
    config::settings::refresh::RefreshSettings,
    use_cases::lib::{
        events::publish::publish,
        refresh::{next_refresh::next_refresh, scheduled::scheduled},
        tasks::{Scheduling, cancel_quietly::cancel_quietly, schedule::schedule},
    },
};
use osiris_domain::{ConnectedAccount, ConnectionStatus, Event, EventType};
use osiris_events::MessageBus;
use osiris_storage::{Store, connected_accounts::NewTokens};

/// What activating a consent needs.
pub struct Activation<'a> {
    pub store: &'a Store,
    pub messages: &'a MessageBus,
    pub scheduling: Scheduling<'a>,
    pub refresh: &'a RefreshSettings,
    /// Whether the connection's provider issues refresh tokens.
    pub issues_refresh_tokens: bool,
}

pub async fn activate_consent(
    activation: Activation<'_>,
    account: &ConnectedAccount,
    tokens: NewTokens,
) -> Outcome {
    let Activation {
        store,
        messages,
        scheduling,
        refresh,
        issues_refresh_tokens,
    } = activation;
    let next = match first_refresh(issues_refresh_tokens, tokens.refresh_token.is_some()) {
        FirstRefresh::Schedule => Some(next_refresh(
            refresh,
            account,
            account.token_generation + 1,
            tokens.access_token_expires_at,
        )),
        FirstRefresh::NotNeeded => None,
        FirstRefresh::MissingRefreshToken => {
            return disable_consent(
                store,
                account,
                "the provider sent no refresh token; check that the app issues them (expiring tokens or token rotation), then connect again",
            )
            .await;
        }
    };
    if let Some(task) = &next
        && let Err(report) = schedule(scheduling, task).await
    {
        tracing::warn!(error = ?report, connected_account_id = %account.id, "the first refresh could not be scheduled; the connection is disabled");
        return disable_consent(
            store,
            account,
            "osiris could not schedule the token refresh; connect again",
        )
        .await;
    }
    let saved = store
        .activate(account, &tokens, next.as_ref().map(scheduled).as_ref())
        .await;
    match saved {
        Ok(()) => {
            publish(
                messages,
                scheduling,
                &Event::new(EventType::ConnectionActive, &account.id),
            )
            .await;
            tracing::info!(connected_account_id = %account.id, "connection active");
            Outcome::Redirect(back_to(
                &account.callback_url,
                &account.id,
                ConnectionStatus::Active,
            ))
        }
        Err(error) => {
            tracing::error!(?error, "saving the tokens failed");
            if let Some(task) = &next {
                cancel_quietly(scheduling, &task.name).await;
            }
            consent_failed(store, account, "osiris could not save the tokens").await
        }
    }
}
