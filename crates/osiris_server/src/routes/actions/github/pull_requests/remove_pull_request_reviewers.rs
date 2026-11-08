//! `POST /v1/connected-accounts/{id}/actions/github.remove_pull_request_reviewers`.

use crate::{
    http::{auth::Auth, error::ApiResult, json_body::JsonBody, server_wrap, state::ApiState},
    use_cases::lib::shared::action_args::ActionArgs,
};
use axum::{
    Json,
    extract::{Path, State},
    http::{HeaderMap, StatusCode},
};
use osiris_api_models::{
    actions::{
        ActionResult,
        github::{GithubReviewRequest, pull_requests::RemovePullRequestReviewers},
    },
    errors::ErrorBody,
};
use osiris_env::Flow;

/// Users and teams removed from those a pull request's review is requested from.
#[utoipa::path(post, path = "/v1/connected-accounts/{id}/actions/github.remove_pull_request_reviewers", tag = "actions", params(("id" = String, Path)), request_body = RemovePullRequestReviewers,
    params(("Idempotency-Key" = Option<String>, Header, description = "Runs the request once; a retry gets the first answer")),
    responses((status = 200, body = ActionResult<GithubReviewRequest>), (status = 403, body = ErrorBody), (status = 404, body = ErrorBody), (status = 409, body = ErrorBody), (status = 424, body = ErrorBody), (status = 502, body = ErrorBody)),
    security(("api_key" = [])))]
pub async fn github_remove_pull_request_reviewers(
    State(state): State<ApiState>,
    headers: HeaderMap,
    Path(id): Path<String>,
    JsonBody(input): JsonBody<RemovePullRequestReviewers>,
) -> ApiResult<(StatusCode, Json<ActionResult<GithubReviewRequest>>)> {
    server_wrap(
        Flow::ActionsRun,
        &state,
        &headers,
        Auth::Api,
        StatusCode::OK,
        |use_cases| async move {
            use_cases
                .github_remove_pull_request_reviewers(ActionArgs {
                    id: &id,
                    input: &input,
                })
                .await
                .map(|data| ActionResult { data })
        },
    )
    .await
}
