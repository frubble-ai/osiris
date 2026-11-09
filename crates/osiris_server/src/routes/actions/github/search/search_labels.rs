//! `POST /v1/connected-accounts/{id}/actions/github.search_labels`.

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
        github::{GithubLabelSearchResults, search::SearchLabels},
    },
    errors::ErrorBody,
};
use osiris_env::Flow;

/// A repository's labels whose name or description match a search, a page at a time.
#[utoipa::path(post, path = "/v1/connected-accounts/{id}/actions/github.search_labels", tag = "actions", params(("id" = String, Path)), request_body = SearchLabels,
    params(("Idempotency-Key" = Option<String>, Header, description = "Runs the request once; a retry gets the first answer")),
    responses((status = 200, body = ActionResult<GithubLabelSearchResults>), (status = 403, body = ErrorBody), (status = 404, body = ErrorBody), (status = 409, body = ErrorBody), (status = 424, body = ErrorBody), (status = 502, body = ErrorBody)),
    security(("api_key" = [])))]
pub async fn github_search_labels(
    State(state): State<ApiState>,
    headers: HeaderMap,
    Path(id): Path<String>,
    JsonBody(input): JsonBody<SearchLabels>,
) -> ApiResult<(StatusCode, Json<ActionResult<GithubLabelSearchResults>>)> {
    server_wrap(
        Flow::ActionsRun,
        &state,
        &headers,
        Auth::Api,
        StatusCode::OK,
        |use_cases| async move {
            use_cases
                .github_search_labels(ActionArgs {
                    id: &id,
                    input: &input,
                })
                .await
                .map(|data| ActionResult { data })
        },
    )
    .await
}
