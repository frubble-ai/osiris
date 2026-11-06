//! A provider's token answer as tokens to db, with the expiry made absolute.

use crate::use_cases::lib::connected_accounts::NewTokens;
use osiris_interfaces::Tokens;
use time::{Duration, OffsetDateTime};

pub fn new_tokens(tokens: Tokens) -> NewTokens {
    let now = OffsetDateTime::now_utc();
    NewTokens {
        access_token: tokens.access_token,
        refresh_token: tokens.refresh_token,
        access_token_expires_at: tokens
            .expires_in
            .map(|seconds| now + Duration::seconds(seconds)),
        scope: tokens.scope,
    }
}
