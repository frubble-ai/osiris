//! Slack's answers as it sends them, and into our types. Every Web API answer says `ok`; a failed
//! one names its `error`.

use super::types::SlackChannel;
use crate::ProviderError;
use serde::Deserialize;

/// The errors that mean the token is no longer accepted.
const TOKEN_GONE: &[&str] = &[
    "invalid_auth",
    "token_revoked",
    "account_inactive",
    "token_expired",
    "not_authed",
];

/// The errors that mean Slack failed for now: try again, the grant is not at fault.
const TRY_AGAIN: &[&str] = &[
    "ratelimited",
    "service_unavailable",
    "internal_error",
    "fatal_error",
    "request_timeout",
];

/// `ok`, and the `error` when not.
#[derive(Deserialize)]
pub struct Status {
    pub ok: bool,
    pub error: Option<String>,
}

impl Status {
    /// A failed answer as a [`ProviderError`]; none when it worked.
    pub fn failure(&self, method: &str) -> Option<ProviderError> {
        if self.ok {
            return None;
        }
        let error = self.error.as_deref().unwrap_or("unknown");
        let message = format!("{method}: {error}");
        Some(if TOKEN_GONE.contains(&error) {
            ProviderError::unauthorized("slack", message)
        } else if TRY_AGAIN.contains(&error) {
            ProviderError::unavailable("slack", message)
        } else {
            ProviderError::refused("slack", message)
        })
    }

    /// The token was already gone: as good as revoked.
    pub fn token_gone(&self) -> bool {
        self.error
            .as_deref()
            .is_some_and(|error| TOKEN_GONE.contains(&error))
    }
}

/// A page of `conversations.list`.
#[derive(Deserialize)]
pub struct ChannelsPage {
    #[serde(flatten)]
    pub status: Status,
    #[serde(default)]
    channels: Vec<ChannelAnswer>,
    response_metadata: Option<ResponseMetadata>,
}

#[derive(Deserialize)]
struct ChannelAnswer {
    id: String,
    name: Option<String>,
    #[serde(default)]
    is_private: bool,
    num_members: Option<u64>,
}

#[derive(Deserialize)]
struct ResponseMetadata {
    next_cursor: Option<String>,
}

impl ChannelsPage {
    /// The next page's cursor; none on the last.
    pub fn next_cursor(&self) -> Option<String> {
        self.response_metadata
            .as_ref()
            .and_then(|metadata| metadata.next_cursor.clone())
            .filter(|cursor| !cursor.is_empty())
    }

    pub fn channels(self) -> impl Iterator<Item = SlackChannel> {
        self.channels.into_iter().map(|answer| SlackChannel {
            // A channel without a name shows its id.
            name: answer.name.unwrap_or_else(|| answer.id.clone()),
            id: answer.id,
            is_private: answer.is_private,
            num_members: answer.num_members,
        })
    }
}
