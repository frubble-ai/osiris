//! The consent finished, but the connection can't be kept fresh: no tokens are kept, the
//! connection is disabled, and the browser goes back to the client with `status=disabled`.

use super::{Outcome, back_to::back_to};
use osiris_domain::{ConnectedAccount, ConnectionStatus};
use osiris_storage::Store;

pub async fn disable_consent(store: &Store, account: &ConnectedAccount, reason: &str) -> Outcome {
    tracing::warn!(connected_account_id = %account.id, %reason, "connection disabled at connect");
    if let Err(error) = store.disable_consent(&account.id, reason).await {
        tracing::error!(?error, "disabling the connection did not work");
    }
    Outcome::Redirect(back_to(
        &account.callback_url,
        &account.id,
        ConnectionStatus::Disabled,
    ))
}
