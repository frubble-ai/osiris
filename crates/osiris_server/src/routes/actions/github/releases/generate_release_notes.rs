//! `POST /v1/connected-accounts/{id}/actions/github.generate_release_notes`.

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
        github::{GithubReleaseNotes, releases::GenerateReleaseNotes},
    },
    errors::ErrorBody,
};
use osiris_env::Flow;

/// The name and body GitHub writes for a release of a tag. Nothing is saved.
#[utoipa::path(post, path = "/v1/connected-accounts/{id}/actions/github.generate_release_notes", tag = "actions", params(("id" = String, Path)), request_body = GenerateReleaseNotes,
    params(("Idempotency-Key" = Option<String>, Header, description = "Runs the request once; a retry gets the first answer")),
    responses((status = 200, body = ActionResult<GithubReleaseNotes>), (status = 403, body = ErrorBody), (status = 404, body = ErrorBody), (status = 409, body = ErrorBody), (status = 424, body = ErrorBody), (status = 502, body = ErrorBody)),
    security(("api_key" = [])))]
pub async fn github_generate_release_notes(
    State(state): State<ApiState>,
    headers: HeaderMap,
    Path(id): Path<String>,
    JsonBody(input): JsonBody<GenerateReleaseNotes>,
) -> ApiResult<(StatusCode, Json<ActionResult<GithubReleaseNotes>>)> {
    server_wrap(
        Flow::ActionsRun,
        &state,
        &headers,
        Auth::Api,
        StatusCode::OK,
        |use_cases| async move {
            use_cases
                .github_generate_release_notes(GithubActionArgs {
                    id: &id,
                    input: &input,
                })
                .await
                .map(|data| ActionResult { data })
        },
    )
    .await
}
