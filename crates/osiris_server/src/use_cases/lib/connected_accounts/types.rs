//! What goes into and comes out of the connected-account operations.

use osiris_common::secret::Secret;
use osiris_domain::ConnectedAccount;
use time::OffsetDateTime;

/// A connection whose consent has just been opened.
pub struct NewPending {
    pub auth_config_id: String,
    pub user_id: String,
    pub alias: Option<String>,
    pub callback_url: String,
    pub oauth_state: String,
    pub oauth_state_expires_at: OffsetDateTime,
    pub pkce_verifier: Option<Secret>,
}

/// Tokens a provider just answered.
pub struct NewTokens {
    pub access_token: Secret,
    /// None keeps the refresh token already held.
    pub refresh_token: Option<Secret>,
    pub access_token_expires_at: Option<OffsetDateTime>,
    pub scope: Option<String>,
}

/// A consent the provider sent back: the connection it belongs to, and its PKCE verifier.
pub struct Claimed {
    pub account: ConnectedAccount,
    pub pkce_verifier: Option<Secret>,
    /// The consent came back after its time.
    pub expired: bool,
}

/// The refresh task scheduled for a set of tokens: its name and when it will run.
#[derive(Debug, Clone)]
pub struct ScheduledRefresh {
    pub name: String,
    pub at: OffsetDateTime,
}
