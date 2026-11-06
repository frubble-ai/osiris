//! `POST /v1/connected-accounts/{id}/refresh`.

use crate::http::{auth::Auth, error::ApiResult, server_wrap, state::ApiState};
use crate::use_cases::cases::refresh_connected_account_now::RefreshConnectedAccountNowArgs;
use axum::{
    Json,
    extract::{Path, State},
    http::{HeaderMap, StatusCode},
};
use osiris_api_models::{connected_accounts::ConnectedAccount, errors::ErrorBody};
use osiris_env::Flow;

/// Refresh a connection's tokens now. The immediate refresh task replaces the connection's
/// scheduled one; the answer shows it as `refresh_task`. For operators.
#[utoipa::path(post, path = "/v1/connected-accounts/{id}/refresh", tag = "connected-accounts", params(("id" = String, Path)),
    params(("Idempotency-Key" = Option<String>, Header, description = "Runs the request once; a retry gets the first answer")),
    responses((status = 202, body = ConnectedAccount), (status = 404, body = ErrorBody), (status = 409, body = ErrorBody)), security(("admin_key" = [])))]
pub async fn refresh_connected_account(
    State(state): State<ApiState>,
    headers: HeaderMap,
    Path(id): Path<String>,
) -> ApiResult<(StatusCode, Json<ConnectedAccount>)> {
    server_wrap(
        Flow::ConnectedAccountsRefresh,
        &state,
        &headers,
        Auth::Admin,
        StatusCode::ACCEPTED,
        |use_cases| async move {
            use_cases
                .refresh_connected_account_now(RefreshConnectedAccountNowArgs { id: &id })
                .await
                .map(ConnectedAccount::from)
        },
    )
    .await
}
