//! Builds the refresh task for a set of tokens. It runs a margin before the access token expires,
//! but never later than the keepalive delay, so a token that does not expire is still refreshed
//! regularly. It never runs sooner than the minimum delay.

use crate::config::settings::refresh::RefreshSettings;
use osiris_domain::ConnectedAccount;
use osiris_tasks::{RefreshReason, RefreshTask, Task, TaskPayload, names};
use time::OffsetDateTime;

/// The task that refreshes `account`'s tokens of `generation`, whose access token expires at
/// `expires_at` (`None` if it does not expire).
pub fn next_refresh(
    refresh: &RefreshSettings,
    account: &ConnectedAccount,
    generation: i64,
    expires_at: Option<OffsetDateTime>,
) -> Task {
    let now = OffsetDateTime::now_utc();
    let keepalive_at = now + refresh.keepalive();
    let (run_at, reason) = match expires_at.map(|at| at - refresh.margin()) {
        Some(at) if at < keepalive_at => (at, RefreshReason::Expiry),
        _ => (keepalive_at, RefreshReason::Keepalive),
    };
    Task {
        name: names::refresh(&account.id, generation),
        run_at: run_at.max(now + refresh.min_lead()),
        payload: TaskPayload::ConnectionRefresh(RefreshTask {
            tenant_id: account.user_id.clone(),
            connected_account_id: account.id.clone(),
            generation,
            reason,
        }),
    }
}
