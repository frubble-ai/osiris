//! `GET /v1/auth-configs`: the providers' apps, in batches.

use crate::{
    http::{auth::Auth, error::ApiResult, server_wrap, state::ApiState},
    routes::{cursor::cursor, list::list},
};
use axum::{
    Json,
    extract::{Query, State},
    http::{HeaderMap, StatusCode},
};
use osiris_api_models::{
    auth_configs::AuthConfig,
    list::{List, ListParams},
};
use osiris_env::Flow;

/// The providers' apps, in batches.
#[utoipa::path(get, path = "/v1/auth-configs", tag = "auth-configs", params(ListParams),
    responses((status = 200, body = List<AuthConfig>)), security(("admin_key" = [])))]
pub async fn list_auth_configs(
    State(state): State<ApiState>,
    headers: HeaderMap,
    Query(params): Query<ListParams>,
) -> ApiResult<(StatusCode, Json<List<AuthConfig>>)> {
    let cursor = cursor(&state, params.after, params.before, params.limit);
    server_wrap(
        Flow::AuthConfigsList,
        &state,
        &headers,
        Auth::Admin,
        StatusCode::OK,
        |core| async move { core.list_auth_configs(&cursor).await.map(list) },
    )
    .await
}
