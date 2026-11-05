//! `GET /v1/connected-accounts/{id}`.

use crate::http::{auth::Auth, error::ApiResult, server_wrap, state::ApiState};
use axum::{
    Json,
    extract::{Path, State},
    http::{HeaderMap, StatusCode},
};
use osiris_api_models::{connected_accounts::ConnectedAccount, errors::ErrorBody};
use osiris_env::Flow;

/// One connection, without its tokens.
#[utoipa::path(get, path = "/v1/connected-accounts/{id}", tag = "connected-accounts", params(("id" = String, Path)),
    responses((status = 200, body = ConnectedAccount), (status = 404, body = ErrorBody)), security(("api_key" = [])))]
pub async fn retrieve_connected_account(
    State(state): State<ApiState>,
    headers: HeaderMap,
    Path(id): Path<String>,
) -> ApiResult<(StatusCode, Json<ConnectedAccount>)> {
    server_wrap(
        Flow::ConnectedAccountsRetrieve,
        &state,
        &headers,
        Auth::Api,
        StatusCode::OK,
        |core| async move {
            core.connected_account(&id)
                .await
                .map(ConnectedAccount::from)
        },
    )
    .await
}
