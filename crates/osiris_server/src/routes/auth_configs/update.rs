//! `PATCH /v1/auth-configs/{id}`.

use crate::{
    core::auth_configs::UpdateAuthConfig as Changes,
    http::{auth::Auth, error::ApiResult, server_wrap, state::ApiState},
};
use axum::{
    Json,
    extract::{Path, State},
    http::{HeaderMap, StatusCode},
};
use osiris_api_models::{
    auth_configs::{AuthConfig, UpdateAuthConfig},
    errors::ErrorBody,
};
use osiris_common::secret::secret;
use osiris_env::Flow;

/// Change an app's client, secret or scopes. Connections made before keep the scopes they got.
#[utoipa::path(patch, path = "/v1/auth-configs/{id}", tag = "auth-configs", params(("id" = String, Path)), request_body = UpdateAuthConfig,
    responses((status = 200, body = AuthConfig), (status = 404, body = ErrorBody)), security(("admin_key" = [])))]
pub async fn update_auth_config(
    State(state): State<ApiState>,
    headers: HeaderMap,
    Path(id): Path<String>,
    Json(body): Json<UpdateAuthConfig>,
) -> ApiResult<(StatusCode, Json<AuthConfig>)> {
    let changes = Changes {
        client_id: body.client_id,
        client_secret: body.client_secret.map(secret),
        scopes: body.scopes,
    };
    server_wrap(
        Flow::AuthConfigsUpdate,
        &state,
        &headers,
        Auth::Admin,
        StatusCode::OK,
        |core| async move {
            core.update_auth_config(&id, changes)
                .await
                .map(AuthConfig::from)
        },
    )
    .await
}
