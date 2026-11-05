//! `POST /v1/events/trigger`.

use crate::http::{
    auth::Auth,
    error::{ApiError, ApiResult},
    server_wrap,
    state::ApiState,
};
use axum::{
    Json,
    extract::State,
    http::{HeaderMap, StatusCode},
};
use osiris_api_models::{
    errors::ErrorBody,
    events::{TriggerEvent, TriggeredEvent},
};
use osiris_domain::EventType;
use osiris_env::Flow;

/// Send a test event for a connection, its state left as it is.
#[utoipa::path(post, path = "/v1/events/trigger", tag = "events", request_body = TriggerEvent,
    params(("Idempotency-Key" = Option<String>, Header, description = "Runs the request once; a retry gets the first answer")),
    responses((status = 201, body = TriggeredEvent), (status = 404, body = ErrorBody)), security(("admin_key" = [])))]
pub async fn trigger_event(
    State(state): State<ApiState>,
    headers: HeaderMap,
    Json(body): Json<TriggerEvent>,
) -> ApiResult<(StatusCode, Json<TriggeredEvent>)> {
    let event_type = EventType::parse(&body.event_type)
        .ok_or_else(|| ApiError::Invalid(format!("unknown event type {:?}", body.event_type)))?;
    server_wrap(
        Flow::EventsTrigger,
        &state,
        &headers,
        Auth::Admin,
        StatusCode::CREATED,
        |core| async move {
            let event = core
                .trigger_event(event_type, &body.connected_account_id)
                .await?;
            Ok(TriggeredEvent {
                event_id: event.id,
                event_type: event.event_type.as_str().to_owned(),
                connected_account_id: event.connected_account_id,
            })
        },
    )
    .await
}
