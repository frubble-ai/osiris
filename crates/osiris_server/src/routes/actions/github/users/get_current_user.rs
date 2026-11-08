//! `POST /v1/connected-accounts/{id}/actions/github.get_current_user`.

use crate::{
    http::{auth::Auth, error::ApiResult, server_wrap, state::ApiState},
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
        github::{GithubCurrentUser, users::GetCurrentUser},
    },
    errors::ErrorBody,
};
use osiris_env::Flow;

/// The authenticated user and their profile.
#[utoipa::path(post, path = "/v1/connected-accounts/{id}/actions/github.get_current_user", tag = "actions", params(("id" = String, Path)),
    params(("Idempotency-Key" = Option<String>, Header, description = "Runs the request once; a retry gets the first answer")),
    responses((status = 200, body = ActionResult<GithubCurrentUser>), (status = 403, body = ErrorBody), (status = 404, body = ErrorBody), (status = 409, body = ErrorBody), (status = 424, body = ErrorBody), (status = 502, body = ErrorBody)),
    security(("api_key" = [])))]
pub async fn github_get_current_user(
    State(state): State<ApiState>,
    headers: HeaderMap,
    Path(id): Path<String>,
) -> ApiResult<(StatusCode, Json<ActionResult<GithubCurrentUser>>)> {
    server_wrap(
        Flow::ActionsRun,
        &state,
        &headers,
        Auth::Api,
        StatusCode::OK,
        |use_cases| async move {
            use_cases
                .github_get_current_user(GithubActionArgs {
                    id: &id,
                    input: &GetCurrentUser::default(),
                })
                .await
                .map(|data| ActionResult { data })
        },
    )
    .await
}
