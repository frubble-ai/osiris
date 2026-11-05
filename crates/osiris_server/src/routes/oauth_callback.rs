//! `/oauth/callback`: where providers send the browser after their consent. Public; the `state`
//! is the only thing trusted, and it works once.

use crate::{
    core::oauth_callback::{Callback, Outcome},
    http::state::ApiState,
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
    match state.core.finish_consent(Callback { state: query.state, code: query.code, error: query.error }).await {
        Outcome::Redirect(url) => Redirect::to(&url).into_response(),
        Outcome::Unknown => (
            StatusCode::BAD_REQUEST,
            Html("<!doctype html><title>Osiris</title><p>This connection link is no longer valid. Start connecting again from the app.</p>"),
        )
            .into_response(),
    }
}
