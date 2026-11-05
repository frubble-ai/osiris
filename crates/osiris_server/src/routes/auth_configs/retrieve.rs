//! `GET /v1/auth-configs/{id}`.

use crate::http::{auth::Auth, error::ApiResult, server_wrap, state::ApiState};
use axum::{
    Json,
    extract::{Path, State},
    http::{HeaderMap, StatusCode},
};
use osiris_api_models::{auth_configs::AuthConfig, errors::ErrorBody};
use osiris_env::Flow;

/// One provider's app, without its secret.
#[utoipa::path(get, path = "/v1/auth-configs/{id}", tag = "auth-configs", params(("id" = String, Path)),
    responses((status = 200, body = AuthConfig), (status = 404, body = ErrorBody)), security(("admin_key" = [])))]
pub async fn retrieve_auth_config(
    State(state): State<ApiState>,
    headers: HeaderMap,
    Path(id): Path<String>,
) -> ApiResult<(StatusCode, Json<AuthConfig>)> {
    server_wrap(
        Flow::AuthConfigsRetrieve,
        &state,
        &headers,
        Auth::Admin,
        StatusCode::OK,
        |core| async move { core.auth_config(&id).await.map(AuthConfig::from) },
    )
    .await
}
