//! One user's connection to a provider through an auth config.

use crate::{ConnectionStatus, TokenSet};
use time::OffsetDateTime;

#[derive(Debug, Clone)]
pub struct ConnectedAccount {
    /// `ca_…`.
    pub id: String,
    pub auth_config_id: String,
    pub provider: String,
    /// The client's: who the connection is for.
    pub user_id: String,
    /// The client's own mark for the attempt that made it.
    pub alias: Option<String>,
    pub status: ConnectionStatus,
    pub callback_url: String,
    /// None until the consent finishes.
    pub tokens: Option<TokenSet>,
    pub last_refresh_error: Option<String>,
    /// The number of failed refreshes since the last successful one.
    pub refresh_failures: i32,
    /// Goes up by one every time new tokens are stored. A refresh task records the generation it
    /// was created for, and does nothing if the connection has moved to another one.
    pub token_generation: i64,
    /// The name of the pending refresh task and when it will run. Both are saved in the same
    /// write as the tokens the task will refresh.
    pub refresh_task: Option<String>,
    pub refresh_task_at: Option<OffsetDateTime>,
    pub created_at: OffsetDateTime,
    pub updated_at: OffsetDateTime,
}

impl ConnectedAccount {
    /// Whether setup is complete: the connection is active, holds tokens, and, when it holds a
    /// refresh token, has its refresh task scheduled. A connection to a provider that issues no
    /// refresh tokens (its tokens never expire) is complete with its access token alone; osiris
    /// never activates a connection to a provider that does issue them without one. A connection
    /// that is not complete is never reported as connected.
    pub fn is_complete(&self) -> bool {
        self.status == ConnectionStatus::Active
            && self
                .tokens
                .as_ref()
                .is_some_and(|tokens| tokens.refresh_token.is_none() || self.refresh_task.is_some())
    }
}
