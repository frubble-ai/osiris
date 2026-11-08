//! `POST /v1/connected-accounts/{id}/actions/github.list_issue_comments`.

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
        github::{GithubIssueComment, issues::ListIssueComments},
    },
    errors::ErrorBody,
};
use osiris_env::Flow;

/// The comments on an issue, a page at a time.
#[utoipa::path(post, path = "/v1/connected-accounts/{id}/actions/github.list_issue_comments", tag = "actions", params(("id" = String, Path)), request_body = ListIssueComments,
    params(("Idempotency-Key" = Option<String>, Header, description = "Runs the request once; a retry gets the first answer")),
    responses((status = 200, body = ActionResult<Vec<GithubIssueComment>>), (status = 403, body = ErrorBody), (status = 404, body = ErrorBody), (status = 409, body = ErrorBody), (status = 424, body = ErrorBody), (status = 502, body = ErrorBody)),
    security(("api_key" = [])))]
pub async fn github_list_issue_comments(
    State(state): State<ApiState>,
    headers: HeaderMap,
    Path(id): Path<String>,
    JsonBody(input): JsonBody<ListIssueComments>,
) -> ApiResult<(StatusCode, Json<ActionResult<Vec<GithubIssueComment>>>)> {
    server_wrap(
        Flow::ActionsRun,
        &state,
        &headers,
        Auth::Api,
        StatusCode::OK,
        |use_cases| async move {
            use_cases
                .github_list_issue_comments(GithubActionArgs {
                    id: &id,
                    input: &input,
                })
                .await
                .map(|data| ActionResult { data })
        },
    )
    .await
}
