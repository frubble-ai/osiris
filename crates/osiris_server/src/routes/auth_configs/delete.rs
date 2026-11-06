//! `DELETE /v1/auth-configs/{id}`.

use crate::http::{auth::Auth, error::ApiResult, server_wrap, state::ApiState};
use crate::use_cases::cases::delete_auth_config::DeleteAuthConfigArgs;
use axum::{
    Json,
    extract::{Path, State},
    http::{HeaderMap, StatusCode},
};
use osiris_api_models::{auth_configs::Deleted, errors::ErrorBody};
use osiris_env::Flow;

/// Delete an app no connection uses.
#[utoipa::path(delete, path = "/v1/auth-configs/{id}", tag = "auth-configs", params(("id" = String, Path)),
    params(("Idempotency-Key" = Option<String>, Header, description = "Runs the request once; a retry gets the first answer")),
    responses((status = 200, body = Deleted), (status = 409, body = ErrorBody)), security(("admin_key" = [])))]
pub async fn delete_auth_config(
    State(state): State<ApiState>,
    headers: HeaderMap,
    Path(id): Path<String>,
) -> ApiResult<(StatusCode, Json<Deleted>)> {
    server_wrap(
        Flow::AuthConfigsDelete,
        &state,
        &headers,
        Auth::Admin,
        StatusCode::OK,
        |use_cases| async move {
            use_cases
                .delete_auth_config(DeleteAuthConfigArgs { id: &id })
                .await
                .map(|()| Deleted { id, deleted: true })
        },
    )
    .await
}
