//! `/v1/connected-accounts`.

use osiris_domain::{ConnectionStatus, CredentialKind};
use serde::{Deserialize, Serialize};
use time::OffsetDateTime;
use utoipa::{IntoParams, ToSchema};

#[derive(Debug, Deserialize, Serialize, ToSchema)]
pub struct CreateConnectedAccount {
    /// `ac_…`: the provider's app to connect through.
    pub auth_config_id: String,
    /// Yours: who the connection is for, such as an organization's id.
    pub user_id: String,
    /// Yours: a mark for this attempt, to find what it made if its answer is lost.
    pub alias: Option<String>,
    /// Where the browser goes after the provider's consent, with `connected_account_id` and
    /// `status` added to its query. Required for a consent; left out with `installation_id`.
    pub callback_url: Option<String>,
    /// Connect as this installation of the auth config's app instead of through a user's
    /// consent (the provider's id for the installation): no browser, active at once. The auth config
    /// must hold the app's credentials in its settings.
    pub installation_id: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct ConnectedAccountCreated {
    /// `ca_…`.
    pub id: String,
    #[schema(value_type = String, example = "pending")]
    pub status: ConnectionStatus,
    /// The provider's own consent page: send the browser here. Absent for an installation
    /// connection, which needs no consent.
    pub redirect_url: Option<String>,
}

/// A connection, without its tokens.
#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct ConnectedAccount {
    pub id: String,
    pub auth_config_id: String,
    pub provider: String,
    pub user_id: String,
    pub alias: Option<String>,
    /// `pending` (the consent is open), `active`, `disabled` (no refresh is scheduled, so the
    /// connection can't be used: after a failed connect the user must connect again; after a
    /// refresh, osiris schedules it again by itself), `expired` (the provider refused a refresh
    /// or the grant is gone: reconnect) or `failed` (the consent failed or was never finished). A
    /// connection is only reported `active` when its setup is complete.
    #[schema(value_type = String, example = "active")]
    pub status: ConnectionStatus,
    /// What the connection acts as: `oauth` (the user who consented) or `app_installation` (an
    /// installation of the auth config's app, its tokens minted when an action runs). An open
    /// string.
    #[schema(value_type = String, example = "oauth")]
    pub credential_kind: CredentialKind,
    /// The installation an `app_installation` connection acts as.
    pub installation_id: Option<String>,
    /// Whether setup is complete: osiris holds the tokens and, when they can be refreshed, their
    /// refresh is scheduled. When false, the connection can't be used.
    pub setup_complete: bool,
    /// The pending refresh task's name, and when it runs.
    pub refresh_task: Option<String>,
    #[serde(with = "time::serde::rfc3339::option")]
    #[schema(value_type = Option<String>, format = DateTime)]
    pub refresh_task_at: Option<OffsetDateTime>,
    /// The scopes the provider granted.
    pub scope: Option<String>,
    #[serde(with = "time::serde::rfc3339::option")]
    #[schema(value_type = Option<String>, format = DateTime)]
    pub access_token_expires_at: Option<OffsetDateTime>,
    /// Why the last refresh or the consent failed.
    pub last_error: Option<String>,
    #[serde(with = "time::serde::rfc3339")]
    #[schema(value_type = String, format = DateTime)]
    pub created_at: OffsetDateTime,
    #[serde(with = "time::serde::rfc3339")]
    #[schema(value_type = String, format = DateTime)]
    pub updated_at: OffsetDateTime,
}

impl From<osiris_domain::ConnectedAccount> for ConnectedAccount {
    fn from(account: osiris_domain::ConnectedAccount) -> Self {
        let setup_complete = account.is_complete();
        // Never report a half-connected connection as connected.
        let status = match account.status {
            ConnectionStatus::Active if !setup_complete => ConnectionStatus::Disabled,
            status => status,
        };
        let (scope, expires_at) = account.tokens.map_or((None, None), |tokens| {
            (tokens.scope, tokens.access_token_expires_at)
        });
        Self {
            id: account.id,
            auth_config_id: account.auth_config_id,
            provider: account.provider,
            user_id: account.user_id,
            alias: account.alias,
            status,
            credential_kind: account.credential_kind,
            installation_id: account.installation_id,
            setup_complete,
            refresh_task: account.refresh_task,
            refresh_task_at: account.refresh_task_at,
            scope,
            access_token_expires_at: expires_at,
            last_error: account.last_refresh_error,
            created_at: account.created_at,
            updated_at: account.updated_at,
        }
    }
}

#[derive(Debug, Deserialize, IntoParams)]
#[into_params(parameter_in = Query)]
pub struct ConnectedAccountFilter {
    pub user_id: Option<String>,
    pub provider: Option<String>,
    pub auth_config_id: Option<String>,
    pub alias: Option<String>,
    /// `pending`, `active`, `disabled`, `expired` or `failed`.
    pub status: Option<String>,
    /// A batch's `next_cursor`: the batch after it.
    pub after: Option<String>,
    /// A batch's `previous_cursor`: the batch before it.
    pub before: Option<String>,
    pub limit: Option<i64>,
}
