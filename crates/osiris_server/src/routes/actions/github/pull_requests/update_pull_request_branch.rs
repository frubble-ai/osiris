//! `POST /v1/connected-accounts/{id}/actions/github.update_pull_request_branch`.

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
        github::{GithubPullRequestBranchUpdate, pull_requests::UpdatePullRequestBranch},
    },
    errors::ErrorBody,
};
use osiris_env::Flow;

/// A pull request's branch brought up to date with the latest changes of its base branch.
#[utoipa::path(post, path = "/v1/connected-accounts/{id}/actions/github.update_pull_request_branch", tag = "actions", params(("id" = String, Path)), request_body = UpdatePullRequestBranch,
    params(("Idempotency-Key" = Option<String>, Header, description = "Runs the request once; a retry gets the first answer")),
    responses((status = 200, body = ActionResult<GithubPullRequestBranchUpdate>), (status = 403, body = ErrorBody), (status = 404, body = ErrorBody), (status = 409, body = ErrorBody), (status = 424, body = ErrorBody), (status = 502, body = ErrorBody)),
    security(("api_key" = [])))]
pub async fn github_update_pull_request_branch(
    State(state): State<ApiState>,
    headers: HeaderMap,
    Path(id): Path<String>,
    JsonBody(input): JsonBody<UpdatePullRequestBranch>,
) -> ApiResult<(
    StatusCode,
    Json<ActionResult<GithubPullRequestBranchUpdate>>,
)> {
    server_wrap(
        Flow::ActionsRun,
        &state,
        &headers,
        Auth::Api,
        StatusCode::OK,
        |use_cases| async move {
            use_cases
                .github_update_pull_request_branch(ActionArgs {
                    id: &id,
                    input: &input,
                })
                .await
                .map(|data| ActionResult { data })
        },
    )
    .await
}
