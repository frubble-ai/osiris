//! `POST /v1/connected-accounts/{id}/actions/github.list_repositories`.

use crate::{
    http::{auth::Auth, error::ApiResult, server_wrap, state::ApiState},
    use_cases::cases::github::github_action_args::GithubActionArgs,
};
use axum::{
    Json,
    extract::{Path, State},
    http::{HeaderMap, StatusCode},
};
use osiris_api_models::{
    actions::{
        ActionResult,
        github::{GithubRepository, installations::ListRepositories},
    },
    errors::ErrorBody,
};
use osiris_env::Flow;

/// Every repository the GitHub App's installations give the connected user, each once.
#[utoipa::path(post, path = "/v1/connected-accounts/{id}/actions/github.list_repositories", tag = "actions", params(("id" = String, Path)),
    params(("Idempotency-Key" = Option<String>, Header, description = "Runs the request once; a retry gets the first answer")),
    responses((status = 200, body = ActionResult<Vec<GithubRepository>>), (status = 403, body = ErrorBody), (status = 404, body = ErrorBody), (status = 409, body = ErrorBody), (status = 424, body = ErrorBody), (status = 502, body = ErrorBody)),
    security(("api_key" = [])))]
pub async fn github_list_repositories(
    State(state): State<ApiState>,
    headers: HeaderMap,
    Path(id): Path<String>,
) -> ApiResult<(StatusCode, Json<ActionResult<Vec<GithubRepository>>>)> {
    server_wrap(
        Flow::ActionsRun,
        &state,
        &headers,
        Auth::Api,
        StatusCode::OK,
        |use_cases| async move {
            use_cases
                .github_list_repositories(GithubActionArgs {
                    id: &id,
                    input: &ListRepositories::default(),
                })
                .await
                .map(|data| ActionResult { data })
        },
    )
    .await
}
