//! `POST /v1/connected-accounts/{id}/actions/slack.list_channels`.

use crate::{
    http::{auth::Auth, error::ApiResult, server_wrap, state::ApiState},
    use_cases::cases::list_slack_channels::ListSlackChannelsArgs,
};
use axum::{
    Json,
    extract::{Path, State},
    http::{HeaderMap, StatusCode},
};
use osiris_api_models::{
    actions::{ActionResult, SlackChannel},
    errors::ErrorBody,
};
use osiris_env::Flow;

/// The Slack channels the connected user can see, archived ones left out.
#[utoipa::path(post, path = "/v1/connected-accounts/{id}/actions/slack.list_channels", tag = "actions", params(("id" = String, Path)),
    params(("Idempotency-Key" = Option<String>, Header, description = "Runs the request once; a retry gets the first answer")),
    responses((status = 200, body = ActionResult<Vec<SlackChannel>>), (status = 404, body = ErrorBody), (status = 409, body = ErrorBody), (status = 424, body = ErrorBody), (status = 502, body = ErrorBody)),
    security(("api_key" = [])))]
pub async fn slack_list_channels(
    State(state): State<ApiState>,
    headers: HeaderMap,
    Path(id): Path<String>,
) -> ApiResult<(StatusCode, Json<ActionResult<Vec<SlackChannel>>>)> {
    server_wrap(
        Flow::ActionsRun,
        &state,
        &headers,
        Auth::Api,
        StatusCode::OK,
        |use_cases| async move {
            use_cases
                .list_slack_channels(ListSlackChannelsArgs { id: &id })
                .await
                .map(|data| ActionResult { data })
        },
    )
    .await
}
