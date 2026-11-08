//! `POST /v1/connected-accounts/{id}/actions/github.delete_milestone`.

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
        github::{GithubAcknowledgement, issues::DeleteMilestone},
    },
    errors::ErrorBody,
};
use osiris_env::Flow;

/// A milestone deleted from a repository.
#[utoipa::path(post, path = "/v1/connected-accounts/{id}/actions/github.delete_milestone", tag = "actions", params(("id" = String, Path)), request_body = DeleteMilestone,
    params(("Idempotency-Key" = Option<String>, Header, description = "Runs the request once; a retry gets the first answer")),
    responses((status = 200, body = ActionResult<GithubAcknowledgement>), (status = 403, body = ErrorBody), (status = 404, body = ErrorBody), (status = 409, body = ErrorBody), (status = 424, body = ErrorBody), (status = 502, body = ErrorBody)),
    security(("api_key" = [])))]
pub async fn github_delete_milestone(
    State(state): State<ApiState>,
    headers: HeaderMap,
    Path(id): Path<String>,
    JsonBody(input): JsonBody<DeleteMilestone>,
) -> ApiResult<(StatusCode, Json<ActionResult<GithubAcknowledgement>>)> {
    server_wrap(
        Flow::ActionsRun,
        &state,
        &headers,
        Auth::Api,
        StatusCode::OK,
        |use_cases| async move {
            use_cases
                .github_delete_milestone(GithubActionArgs {
                    id: &id,
                    input: &input,
                })
                .await
                .map(|data| ActionResult { data })
        },
    )
    .await
}
