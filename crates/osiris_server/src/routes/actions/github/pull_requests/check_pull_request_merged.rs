//! `POST /v1/connected-accounts/{id}/actions/github.check_pull_request_merged`.

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
        github::{GithubMergedCheck, pull_requests::CheckPullRequestMerged},
    },
    errors::ErrorBody,
};
use osiris_env::Flow;

/// Whether a pull request has been merged.
#[utoipa::path(post, path = "/v1/connected-accounts/{id}/actions/github.check_pull_request_merged", tag = "actions", params(("id" = String, Path)), request_body = CheckPullRequestMerged,
    params(("Idempotency-Key" = Option<String>, Header, description = "Runs the request once; a retry gets the first answer")),
    responses((status = 200, body = ActionResult<GithubMergedCheck>), (status = 403, body = ErrorBody), (status = 404, body = ErrorBody), (status = 409, body = ErrorBody), (status = 424, body = ErrorBody), (status = 502, body = ErrorBody)),
    security(("api_key" = [])))]
pub async fn github_check_pull_request_merged(
    State(state): State<ApiState>,
    headers: HeaderMap,
    Path(id): Path<String>,
    JsonBody(input): JsonBody<CheckPullRequestMerged>,
) -> ApiResult<(StatusCode, Json<ActionResult<GithubMergedCheck>>)> {
    server_wrap(
        Flow::ActionsRun,
        &state,
        &headers,
        Auth::Api,
        StatusCode::OK,
        |use_cases| async move {
            use_cases
                .github_check_pull_request_merged(ActionArgs {
                    id: &id,
                    input: &input,
                })
                .await
                .map(|data| ActionResult { data })
        },
    )
    .await
}
