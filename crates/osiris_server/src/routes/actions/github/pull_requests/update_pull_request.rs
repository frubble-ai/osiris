//! `POST /v1/connected-accounts/{id}/actions/github.update_pull_request`.

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
        github::{GithubPullRequest, pull_requests::UpdatePullRequest},
    },
    errors::ErrorBody,
};
use osiris_env::Flow;

/// A pull request's title, text, state, base branch or whether maintainers may push to it, changed.
#[utoipa::path(post, path = "/v1/connected-accounts/{id}/actions/github.update_pull_request", tag = "actions", params(("id" = String, Path)), request_body = UpdatePullRequest,
    params(("Idempotency-Key" = Option<String>, Header, description = "Runs the request once; a retry gets the first answer")),
    responses((status = 200, body = ActionResult<GithubPullRequest>), (status = 403, body = ErrorBody), (status = 404, body = ErrorBody), (status = 409, body = ErrorBody), (status = 424, body = ErrorBody), (status = 502, body = ErrorBody)),
    security(("api_key" = [])))]
pub async fn github_update_pull_request(
    State(state): State<ApiState>,
    headers: HeaderMap,
    Path(id): Path<String>,
    JsonBody(input): JsonBody<UpdatePullRequest>,
) -> ApiResult<(StatusCode, Json<ActionResult<GithubPullRequest>>)> {
    server_wrap(
        Flow::ActionsRun,
        &state,
        &headers,
        Auth::Api,
        StatusCode::OK,
        |use_cases| async move {
            use_cases
                .github_update_pull_request(ActionArgs {
                    id: &id,
                    input: &input,
                })
                .await
                .map(|data| ActionResult { data })
        },
    )
    .await
}
