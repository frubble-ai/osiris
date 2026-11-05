//! `DELETE /v1/connected-accounts/{id}`.

use crate::http::{auth::Auth, error::ApiResult, server_wrap, state::ApiState};
use axum::{
    Json,
    extract::{Path, State},
    http::{HeaderMap, StatusCode},
};
use osiris_api_models::{auth_configs::Deleted, errors::ErrorBody};
use osiris_env::Flow;

/// End the grant at the provider and forget the connection. A grant already gone there counts.
#[utoipa::path(delete, path = "/v1/connected-accounts/{id}", tag = "connected-accounts", params(("id" = String, Path)),
    params(("Idempotency-Key" = Option<String>, Header, description = "Runs the request once; a retry gets the first answer")),
    responses((status = 200, body = Deleted), (status = 404, body = ErrorBody), (status = 502, body = ErrorBody)), security(("api_key" = [])))]
pub async fn delete_connected_account(
    State(state): State<ApiState>,
    headers: HeaderMap,
    Path(id): Path<String>,
) -> ApiResult<(StatusCode, Json<Deleted>)> {
    server_wrap(
        Flow::ConnectedAccountsDelete,
        &state,
        &headers,
        Auth::Api,
        StatusCode::OK,
        |core| async move {
            core.delete_connected_account(&id)
                .await
                .map(|()| Deleted { id, deleted: true })
        },
    )
    .await
}
