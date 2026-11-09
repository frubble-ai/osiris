//! `POST /v1/connected-accounts/{id}/actions/github.list_matching_refs`.

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
        github::{GithubGitRef, repositories::ListMatchingRefs},
    },
    errors::ErrorBody,
};
use osiris_env::Flow;

/// Every git reference that starts with a prefix.
#[utoipa::path(post, path = "/v1/connected-accounts/{id}/actions/github.list_matching_refs", tag = "actions", params(("id" = String, Path)), request_body = ListMatchingRefs,
    params(("Idempotency-Key" = Option<String>, Header, description = "Runs the request once; a retry gets the first answer")),
    responses((status = 200, body = ActionResult<Vec<GithubGitRef>>), (status = 403, body = ErrorBody), (status = 404, body = ErrorBody), (status = 409, body = ErrorBody), (status = 424, body = ErrorBody), (status = 502, body = ErrorBody)),
    security(("api_key" = [])))]
pub async fn github_list_matching_refs(
    State(state): State<ApiState>,
    headers: HeaderMap,
    Path(id): Path<String>,
    JsonBody(input): JsonBody<ListMatchingRefs>,
) -> ApiResult<(StatusCode, Json<ActionResult<Vec<GithubGitRef>>>)> {
    server_wrap(
        Flow::ActionsRun,
        &state,
        &headers,
        Auth::Api,
        StatusCode::OK,
        |use_cases| async move {
            use_cases
                .github_list_matching_refs(ActionArgs {
                    id: &id,
                    input: &input,
                })
                .await
                .map(|data| ActionResult { data })
        },
    )
    .await
}
