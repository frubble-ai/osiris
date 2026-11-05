//! `POST /v1/connected-accounts/{id}/actions/linear.list_teams`.

use crate::{
    http::{auth::Auth, error::ApiResult, server_wrap, state::ApiState},
    use_cases::cases::list_linear_teams::ListLinearTeamsArgs,
};
use axum::{
    Json,
    extract::{Path, State},
    http::{HeaderMap, StatusCode},
};
use osiris_api_models::{
    actions::{ActionResult, LinearTeam},
    errors::ErrorBody,
};
use osiris_env::Flow;

/// The Linear teams the connected user can see.
#[utoipa::path(post, path = "/v1/connected-accounts/{id}/actions/linear.list_teams", tag = "actions", params(("id" = String, Path)),
    params(("Idempotency-Key" = Option<String>, Header, description = "Runs the request once; a retry gets the first answer")),
    responses((status = 200, body = ActionResult<Vec<LinearTeam>>), (status = 409, body = ErrorBody), (status = 424, body = ErrorBody), (status = 502, body = ErrorBody)),
    security(("api_key" = [])))]
pub async fn linear_list_teams(
    State(state): State<ApiState>,
    headers: HeaderMap,
    Path(id): Path<String>,
) -> ApiResult<(StatusCode, Json<ActionResult<Vec<LinearTeam>>>)> {
    server_wrap(
        Flow::ActionsRun,
        &state,
        &headers,
        Auth::Api,
        StatusCode::OK,
        |use_cases| async move {
            use_cases
                .list_linear_teams(ListLinearTeamsArgs { id: &id })
                .await
                .map(|data| ActionResult { data })
        },
    )
    .await
}
