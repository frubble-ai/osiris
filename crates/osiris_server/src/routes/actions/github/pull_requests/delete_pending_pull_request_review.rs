//! `POST /v1/connected-accounts/{id}/actions/github.delete_pending_pull_request_review`.

use crate::{
    http::{auth::Auth, error::ApiResult, json_body::JsonBody, server_wrap, state::ApiState},
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
        github::{GithubReview, pull_requests::DeletePendingPullRequestReview},
    },
    errors::ErrorBody,
};
use osiris_env::Flow;

/// A pending review of a pull request, deleted; it answers the review deleted.
#[utoipa::path(post, path = "/v1/connected-accounts/{id}/actions/github.delete_pending_pull_request_review", tag = "actions", params(("id" = String, Path)), request_body = DeletePendingPullRequestReview,
    params(("Idempotency-Key" = Option<String>, Header, description = "Runs the request once; a retry gets the first answer")),
    responses((status = 200, body = ActionResult<GithubReview>), (status = 403, body = ErrorBody), (status = 404, body = ErrorBody), (status = 409, body = ErrorBody), (status = 424, body = ErrorBody), (status = 502, body = ErrorBody)),
    security(("api_key" = [])))]
pub async fn github_delete_pending_pull_request_review(
    State(state): State<ApiState>,
    headers: HeaderMap,
    Path(id): Path<String>,
    JsonBody(input): JsonBody<DeletePendingPullRequestReview>,
) -> ApiResult<(StatusCode, Json<ActionResult<GithubReview>>)> {
    server_wrap(
        Flow::ActionsRun,
        &state,
        &headers,
        Auth::Api,
        StatusCode::OK,
        |use_cases| async move {
            use_cases
                .github_delete_pending_pull_request_review(GithubActionArgs {
                    id: &id,
                    input: &input,
                })
                .await
                .map(|data| ActionResult { data })
        },
    )
    .await
}
