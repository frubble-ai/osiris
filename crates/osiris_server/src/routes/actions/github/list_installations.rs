//! `POST /v1/connected-accounts/{id}/actions/github.list_installations`.

use crate::{
    http::{auth::Auth, error::ApiResult, server_wrap, state::ApiState},
    use_cases::cases::list_github_installations::ListGithubInstallationsArgs,
};
use axum::{
    Json,
    extract::{Path, State},
    http::{HeaderMap, StatusCode},
};
use osiris_api_models::{
    actions::{ActionResult, GithubInstallation},
    errors::ErrorBody,
};
use osiris_env::Flow;

/// Where the GitHub App is installed, of what the connected user can see.
#[utoipa::path(post, path = "/v1/connected-accounts/{id}/actions/github.list_installations", tag = "actions", params(("id" = String, Path)),
    params(("Idempotency-Key" = Option<String>, Header, description = "Runs the request once; a retry gets the first answer")),
    responses((status = 200, body = ActionResult<Vec<GithubInstallation>>), (status = 404, body = ErrorBody), (status = 409, body = ErrorBody), (status = 424, body = ErrorBody), (status = 502, body = ErrorBody)),
    security(("api_key" = [])))]
pub async fn github_list_installations(
    State(state): State<ApiState>,
    headers: HeaderMap,
    Path(id): Path<String>,
) -> ApiResult<(StatusCode, Json<ActionResult<Vec<GithubInstallation>>>)> {
    server_wrap(
        Flow::ActionsRun,
        &state,
        &headers,
        Auth::Api,
        StatusCode::OK,
        |use_cases| async move {
            use_cases
                .list_github_installations(ListGithubInstallationsArgs { id: &id })
                .await
                .map(|data| ActionResult { data })
        },
    )
    .await
}
