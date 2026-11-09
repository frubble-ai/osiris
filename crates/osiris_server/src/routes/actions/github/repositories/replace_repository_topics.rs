//! `POST /v1/connected-accounts/{id}/actions/github.replace_repository_topics`.

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
        github::{GithubTopics, repositories::ReplaceRepositoryTopics},
    },
    errors::ErrorBody,
};
use osiris_env::Flow;

/// A repository's topics replaced by a new set.
#[utoipa::path(post, path = "/v1/connected-accounts/{id}/actions/github.replace_repository_topics", tag = "actions", params(("id" = String, Path)), request_body = ReplaceRepositoryTopics,
    params(("Idempotency-Key" = Option<String>, Header, description = "Runs the request once; a retry gets the first answer")),
    responses((status = 200, body = ActionResult<GithubTopics>), (status = 403, body = ErrorBody), (status = 404, body = ErrorBody), (status = 409, body = ErrorBody), (status = 424, body = ErrorBody), (status = 502, body = ErrorBody)),
    security(("api_key" = [])))]
pub async fn github_replace_repository_topics(
    State(state): State<ApiState>,
    headers: HeaderMap,
    Path(id): Path<String>,
    JsonBody(input): JsonBody<ReplaceRepositoryTopics>,
) -> ApiResult<(StatusCode, Json<ActionResult<GithubTopics>>)> {
    server_wrap(
        Flow::ActionsRun,
        &state,
        &headers,
        Auth::Api,
        StatusCode::OK,
        |use_cases| async move {
            use_cases
                .github_replace_repository_topics(ActionArgs {
                    id: &id,
                    input: &input,
                })
                .await
                .map(|data| ActionResult { data })
        },
    )
    .await
}
