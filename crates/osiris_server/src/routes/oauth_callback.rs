//! `/oauth/callback`: where providers send the browser after their consent. Public; the `state`
//! is the only thing trusted, and it works once.

use crate::{
    http::state::ApiState,
    use_cases::{
        cases::finish_consent::FinishConsentArgs,
        lib::consent::{Callback, Outcome},
    },
};
use axum::{
    extract::{Query, State},
    http::StatusCode,
    response::{Html, IntoResponse, Redirect, Response},
};
use osiris_env::Flow;
use serde::Deserialize;

#[derive(Deserialize)]
pub struct CallbackQuery {
    state: Option<String>,
    code: Option<String>,
    error: Option<String>,
}

#[tracing::instrument(skip_all, fields(flow = %Flow::OAuthCallback))]
pub async fn oauth_callback(
    State(state): State<ApiState>,
    Query(query): Query<CallbackQuery>,
) -> Response {
    let finished = state
        .use_cases
        .finish_consent(FinishConsentArgs {
            callback: Callback {
                state: query.state,
                code: query.code,
                error: query.error,
            },
        })
        .await;
    match finished {
        Ok(Outcome::Redirect(url)) => Redirect::to(&url).into_response(),
        Ok(Outcome::Unknown) | Err(_) => (
            StatusCode::BAD_REQUEST,
            Html("<!doctype html><title>Osiris</title><p>This connection link is no longer valid. Start connecting again from the app.</p>"),
        )
            .into_response(),
    }
}
