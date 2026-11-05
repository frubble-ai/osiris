//! The consent ended without tokens: the connection fails and the browser goes back to the client
//! with `status=failed`.

use super::{Outcome, back_to::back_to};
use osiris_domain::{ConnectedAccount, ConnectionStatus};
use osiris_storage::Store;

pub async fn consent_failed(store: &Store, account: &ConnectedAccount, reason: &str) -> Outcome {
    tracing::warn!(connected_account_id = %account.id, %reason, "consent failed");
    if let Err(error) = store.fail(&account.id, reason).await {
        tracing::error!(?error, "marking the consent failed did not work");
    }
    Outcome::Redirect(back_to(
        &account.callback_url,
        &account.id,
        ConnectionStatus::Failed,
    ))
}
