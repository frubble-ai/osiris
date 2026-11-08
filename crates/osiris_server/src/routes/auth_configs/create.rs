//! `POST /v1/auth-configs`: register a provider's app.

use crate::{
    http::{auth::Auth, error::ApiResult, json_body::JsonBody, server_wrap, state::ApiState},
    use_cases::cases::create_auth_config::CreateAuthConfigArgs as New,
};
use axum::{
    Json,
    extract::State,
    http::{HeaderMap, StatusCode},
};
use osiris_api_models::{
    auth_configs::{AuthConfig, CreateAuthConfig},
    errors::ErrorBody,
};
use osiris_env::Flow;

/// Register a provider's app.
#[utoipa::path(post, path = "/v1/auth-configs", tag = "auth-configs", request_body = CreateAuthConfig,
    params(("Idempotency-Key" = Option<String>, Header, description = "Runs the request once; a retry gets the first answer")),
    responses((status = 201, body = AuthConfig), (status = 400, body = ErrorBody)), security(("admin_key" = [])))]
pub async fn create_auth_config(
    State(state): State<ApiState>,
    headers: HeaderMap,
    JsonBody(body): JsonBody<CreateAuthConfig>,
) -> ApiResult<(StatusCode, Json<AuthConfig>)> {
    let new = New {
        provider: body.provider,
        client_id: body.client_id,
        client_secret: body.client_secret,
        scopes: body.scopes,
    };
    server_wrap(
        Flow::AuthConfigsCreate,
        &state,
        &headers,
        Auth::Admin,
        StatusCode::CREATED,
        |use_cases| async move {
            use_cases
                .create_auth_config(new)
                .await
                .map(AuthConfig::from)
        },
    )
    .await
}
