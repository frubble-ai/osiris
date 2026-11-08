//! `POST /v1/connected-accounts/{id}/actions/github.list_directory_contents`.

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
        github::{GithubContentEntry, repositories::ListDirectoryContents},
    },
    errors::ErrorBody,
};
use osiris_env::Flow;

/// The entries of a directory of a repository.
#[utoipa::path(post, path = "/v1/connected-accounts/{id}/actions/github.list_directory_contents", tag = "actions", params(("id" = String, Path)), request_body = ListDirectoryContents,
    params(("Idempotency-Key" = Option<String>, Header, description = "Runs the request once; a retry gets the first answer")),
    responses((status = 200, body = ActionResult<Vec<GithubContentEntry>>), (status = 403, body = ErrorBody), (status = 404, body = ErrorBody), (status = 409, body = ErrorBody), (status = 424, body = ErrorBody), (status = 502, body = ErrorBody)),
    security(("api_key" = [])))]
pub async fn github_list_directory_contents(
    State(state): State<ApiState>,
    headers: HeaderMap,
    Path(id): Path<String>,
    JsonBody(input): JsonBody<ListDirectoryContents>,
) -> ApiResult<(StatusCode, Json<ActionResult<Vec<GithubContentEntry>>>)> {
    server_wrap(
        Flow::ActionsRun,
        &state,
        &headers,
        Auth::Api,
        StatusCode::OK,
        |use_cases| async move {
            use_cases
                .github_list_directory_contents(ActionArgs {
                    id: &id,
                    input: &input,
                })
                .await
                .map(|data| ActionResult { data })
        },
    )
    .await
}
