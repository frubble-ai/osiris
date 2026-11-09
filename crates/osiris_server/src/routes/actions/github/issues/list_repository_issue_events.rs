//! `POST /v1/connected-accounts/{id}/actions/github.list_repository_issue_events`.

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
        github::{GithubIssueEvent, issues::ListRepositoryIssueEvents},
    },
    errors::ErrorBody,
};
use osiris_env::Flow;

/// What happened to every issue in a repository, a page at a time.
#[utoipa::path(post, path = "/v1/connected-accounts/{id}/actions/github.list_repository_issue_events", tag = "actions", params(("id" = String, Path)), request_body = ListRepositoryIssueEvents,
    params(("Idempotency-Key" = Option<String>, Header, description = "Runs the request once; a retry gets the first answer")),
    responses((status = 200, body = ActionResult<Vec<GithubIssueEvent>>), (status = 403, body = ErrorBody), (status = 404, body = ErrorBody), (status = 409, body = ErrorBody), (status = 424, body = ErrorBody), (status = 502, body = ErrorBody)),
    security(("api_key" = [])))]
pub async fn github_list_repository_issue_events(
    State(state): State<ApiState>,
    headers: HeaderMap,
    Path(id): Path<String>,
    JsonBody(input): JsonBody<ListRepositoryIssueEvents>,
) -> ApiResult<(StatusCode, Json<ActionResult<Vec<GithubIssueEvent>>>)> {
    server_wrap(
        Flow::ActionsRun,
        &state,
        &headers,
        Auth::Api,
        StatusCode::OK,
        |use_cases| async move {
            use_cases
                .github_list_repository_issue_events(ActionArgs {
                    id: &id,
                    input: &input,
                })
                .await
                .map(|data| ActionResult { data })
        },
    )
    .await
}
