//! `POST /v1/connected-accounts/{id}/actions/github.get_release_asset`.

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
        github::{GithubReleaseAsset, releases::GetReleaseAsset},
    },
    errors::ErrorBody,
};
use osiris_env::Flow;

/// A file attached to a release, by its id.
#[utoipa::path(post, path = "/v1/connected-accounts/{id}/actions/github.get_release_asset", tag = "actions", params(("id" = String, Path)), request_body = GetReleaseAsset,
    params(("Idempotency-Key" = Option<String>, Header, description = "Runs the request once; a retry gets the first answer")),
    responses((status = 200, body = ActionResult<GithubReleaseAsset>), (status = 403, body = ErrorBody), (status = 404, body = ErrorBody), (status = 409, body = ErrorBody), (status = 424, body = ErrorBody), (status = 502, body = ErrorBody)),
    security(("api_key" = [])))]
pub async fn github_get_release_asset(
    State(state): State<ApiState>,
    headers: HeaderMap,
    Path(id): Path<String>,
    JsonBody(input): JsonBody<GetReleaseAsset>,
) -> ApiResult<(StatusCode, Json<ActionResult<GithubReleaseAsset>>)> {
    server_wrap(
        Flow::ActionsRun,
        &state,
        &headers,
        Auth::Api,
        StatusCode::OK,
        |use_cases| async move {
            use_cases
                .github_get_release_asset(ActionArgs {
                    id: &id,
                    input: &input,
                })
                .await
                .map(|data| ActionResult { data })
        },
    )
    .await
}
