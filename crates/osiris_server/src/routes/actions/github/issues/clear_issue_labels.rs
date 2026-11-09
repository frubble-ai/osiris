//! `POST /v1/connected-accounts/{id}/actions/github.clear_issue_labels`.

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
        github::{GithubAcknowledgement, issues::ClearIssueLabels},
    },
    errors::ErrorBody,
};
use osiris_env::Flow;

/// Every label taken off an issue.
#[utoipa::path(post, path = "/v1/connected-accounts/{id}/actions/github.clear_issue_labels", tag = "actions", params(("id" = String, Path)), request_body = ClearIssueLabels,
    params(("Idempotency-Key" = Option<String>, Header, description = "Runs the request once; a retry gets the first answer")),
    responses((status = 200, body = ActionResult<GithubAcknowledgement>), (status = 403, body = ErrorBody), (status = 404, body = ErrorBody), (status = 409, body = ErrorBody), (status = 424, body = ErrorBody), (status = 502, body = ErrorBody)),
    security(("api_key" = [])))]
pub async fn github_clear_issue_labels(
    State(state): State<ApiState>,
    headers: HeaderMap,
    Path(id): Path<String>,
    JsonBody(input): JsonBody<ClearIssueLabels>,
) -> ApiResult<(StatusCode, Json<ActionResult<GithubAcknowledgement>>)> {
    server_wrap(
        Flow::ActionsRun,
        &state,
        &headers,
        Auth::Api,
        StatusCode::OK,
        |use_cases| async move {
            use_cases
                .github_clear_issue_labels(ActionArgs {
                    id: &id,
                    input: &input,
                })
                .await
                .map(|data| ActionResult { data })
        },
    )
    .await
}
