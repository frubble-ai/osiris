//! `PATCH /v1/auth-configs/{id}`.

use crate::{
    http::{auth::Auth, error::ApiResult, json_body::JsonBody, server_wrap, state::ApiState},
    use_cases::cases::update_auth_config::UpdateAuthConfigArgs,
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
use osiris_env::Flow;

/// Change an app's client, secret or scopes. Connections made before keep the scopes they got.
#[utoipa::path(patch, path = "/v1/auth-configs/{id}", tag = "auth-configs", params(("id" = String, Path)), request_body = UpdateAuthConfig,
    responses((status = 200, body = AuthConfig), (status = 404, body = ErrorBody)), security(("admin_key" = [])))]
pub async fn update_auth_config(
    State(state): State<ApiState>,
    headers: HeaderMap,
    Path(id): Path<String>,
    JsonBody(body): JsonBody<UpdateAuthConfig>,
) -> ApiResult<(StatusCode, Json<AuthConfig>)> {
    server_wrap(
        Flow::AuthConfigsUpdate,
        &state,
        &headers,
        Auth::Admin,
        StatusCode::OK,
        |use_cases| async move {
            use_cases
                .update_auth_config(UpdateAuthConfigArgs {
                    id: &id,
                    client_id: body.client_id,
                    client_secret: body.client_secret,
                    scopes: body.scopes,
                    settings: body.settings.map(serde_json::Value::Object),
                    secret_settings: body.secret_settings.map(|settings| settings.0),
                })
                .await
                .map(AuthConfig::from)
        },
    )
    .await
}
