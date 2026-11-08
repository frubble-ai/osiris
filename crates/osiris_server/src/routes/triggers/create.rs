//! `POST /v1/triggers`: watch something at a connection's provider.

use crate::{
    http::{auth::Auth, error::ApiResult, json_body::JsonBody, server_wrap, state::ApiState},
    use_cases::cases::triggers::create_trigger::CreateTriggerArgs,
};
use axum::{
    Json,
    extract::State,
    http::{HeaderMap, StatusCode},
};
use osiris_api_models::{
    errors::ErrorBody,
    triggers::{CreateTrigger, Trigger},
};
use osiris_env::Flow;

/// Create a trigger: osiris subscribes at the connection's provider (a GitHub repository webhook)
/// and publishes each delivery as a `trigger.fired` event.
#[utoipa::path(post, path = "/v1/triggers", tag = "triggers", request_body = CreateTrigger,
    params(("Idempotency-Key" = Option<String>, Header, description = "Runs the request once; a retry gets the first answer")),
    responses((status = 201, body = Trigger), (status = 403, body = ErrorBody), (status = 404, body = ErrorBody), (status = 409, body = ErrorBody), (status = 424, body = ErrorBody), (status = 502, body = ErrorBody)),
    security(("api_key" = [])))]
pub async fn create_trigger(
    State(state): State<ApiState>,
    headers: HeaderMap,
    JsonBody(body): JsonBody<CreateTrigger>,
) -> ApiResult<(StatusCode, Json<Trigger>)> {
    let args = CreateTriggerArgs {
        connected_account_id: body.connected_account_id,
        kind: body.trigger,
        config: serde_json::Value::Object(body.config),
    };
    server_wrap(
        Flow::TriggersCreate,
        &state,
        &headers,
        Auth::Api,
        StatusCode::CREATED,
        |use_cases| async move { use_cases.create_trigger(args).await.map(Trigger::from) },
    )
    .await
}
