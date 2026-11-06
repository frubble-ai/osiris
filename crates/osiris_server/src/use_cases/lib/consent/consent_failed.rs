//! The consent ended without tokens: the connection fails and the browser goes back to the client
//! with `status=failed`.

use super::{Outcome, back_to::back_to};
use crate::use_cases::lib::database::Db;
use osiris_domain::{ConnectedAccount, ConnectionStatus};

pub async fn consent_failed(db: &Db, account: &ConnectedAccount, reason: &str) -> Outcome {
    tracing::warn!(connected_account_id = %account.id, %reason, "consent failed");
    if let Err(error) = db.fail(&account.id, reason).await {
        tracing::error!(?error, "marking the consent failed did not work");
    }
    Outcome::Redirect(back_to(
        &account.callback_url,
        &account.id,
        ConnectionStatus::Failed,
    ))
}
