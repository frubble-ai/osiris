//! `POST /v1/connected-accounts/{id}/actions/github.sync_fork_branch_with_upstream`.

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
        github::{GithubForkSync, repositories::SyncForkBranchWithUpstream},
    },
    errors::ErrorBody,
};
use osiris_env::Flow;

/// A fork's branch brought up to date with its upstream.
#[utoipa::path(post, path = "/v1/connected-accounts/{id}/actions/github.sync_fork_branch_with_upstream", tag = "actions", params(("id" = String, Path)), request_body = SyncForkBranchWithUpstream,
    params(("Idempotency-Key" = Option<String>, Header, description = "Runs the request once; a retry gets the first answer")),
    responses((status = 200, body = ActionResult<GithubForkSync>), (status = 403, body = ErrorBody), (status = 404, body = ErrorBody), (status = 409, body = ErrorBody), (status = 424, body = ErrorBody), (status = 502, body = ErrorBody)),
    security(("api_key" = [])))]
pub async fn github_sync_fork_branch_with_upstream(
    State(state): State<ApiState>,
    headers: HeaderMap,
    Path(id): Path<String>,
    JsonBody(input): JsonBody<SyncForkBranchWithUpstream>,
) -> ApiResult<(StatusCode, Json<ActionResult<GithubForkSync>>)> {
    server_wrap(
        Flow::ActionsRun,
        &state,
        &headers,
        Auth::Api,
        StatusCode::OK,
        |use_cases| async move {
            use_cases
                .github_sync_fork_branch_with_upstream(GithubActionArgs {
                    id: &id,
                    input: &input,
                })
                .await
                .map(|data| ActionResult { data })
        },
    )
    .await
}
