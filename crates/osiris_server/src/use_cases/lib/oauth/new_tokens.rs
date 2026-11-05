//! A provider's token answer as tokens to store, with the expiry made absolute.

use osiris_common::secret::secret;
use osiris_interfaces::Tokens;
use osiris_storage::connected_accounts::NewTokens;
use time::{Duration, OffsetDateTime};

pub fn new_tokens(tokens: Tokens) -> NewTokens {
    let now = OffsetDateTime::now_utc();
    NewTokens {
        access_token: secret(tokens.access_token),
        refresh_token: tokens.refresh_token.map(secret),
        access_token_expires_at: tokens
            .expires_in
            .map(|seconds| now + Duration::seconds(seconds)),
        scope: tokens.scope,
    }
}
