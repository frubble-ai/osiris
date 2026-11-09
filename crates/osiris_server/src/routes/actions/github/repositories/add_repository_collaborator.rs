//! `POST /v1/connected-accounts/{id}/actions/github.add_repository_collaborator`.

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
        github::{GithubCollaboratorAddition, repositories::AddRepositoryCollaborator},
    },
    errors::ErrorBody,
};
use osiris_env::Flow;

/// An account added to a repository as a collaborator, or invited to be one.
#[utoipa::path(post, path = "/v1/connected-accounts/{id}/actions/github.add_repository_collaborator", tag = "actions", params(("id" = String, Path)), request_body = AddRepositoryCollaborator,
    params(("Idempotency-Key" = Option<String>, Header, description = "Runs the request once; a retry gets the first answer")),
    responses((status = 200, body = ActionResult<GithubCollaboratorAddition>), (status = 403, body = ErrorBody), (status = 404, body = ErrorBody), (status = 409, body = ErrorBody), (status = 424, body = ErrorBody), (status = 502, body = ErrorBody)),
    security(("api_key" = [])))]
pub async fn github_add_repository_collaborator(
    State(state): State<ApiState>,
    headers: HeaderMap,
    Path(id): Path<String>,
    JsonBody(input): JsonBody<AddRepositoryCollaborator>,
) -> ApiResult<(StatusCode, Json<ActionResult<GithubCollaboratorAddition>>)> {
    server_wrap(
        Flow::ActionsRun,
        &state,
        &headers,
        Auth::Api,
        StatusCode::OK,
        |use_cases| async move {
            use_cases
                .github_add_repository_collaborator(ActionArgs {
                    id: &id,
                    input: &input,
                })
                .await
                .map(|data| ActionResult { data })
        },
    )
    .await
}
