//! `POST /webhooks/triggers/{id}`: where a provider sends a trigger's deliveries (a webhook it
//! was asked to create). Public; the delivery's signature, with the trigger's own secret, is the
//! only thing trusted. A delivery not signed with it is answered 404, as a trigger that doesn't
//! exist is, so a guess learns nothing. A signed one is answered 202, published or ignored.

use crate::{
    http::state::ApiState, use_cases::cases::receive_trigger_delivery::ReceiveTriggerDeliveryArgs,
};
use axum::{
    body::Bytes,
    extract::{Path, State},
    http::{HeaderMap, StatusCode},
};
use osiris_domain::DomainError;
use osiris_env::Flow;

#[tracing::instrument(skip_all, fields(flow = %Flow::TriggerDelivery, trigger_id = %id))]
pub async fn trigger_delivery(
    State(state): State<ApiState>,
    Path(id): Path<String>,
    headers: HeaderMap,
    body: Bytes,
) -> StatusCode {
    let received = state
        .use_cases
        .receive_trigger_delivery(ReceiveTriggerDeliveryArgs {
            trigger_id: &id,
            headers: &headers,
            body: &body,
        })
        .await;
    match received {
        Ok(_) => StatusCode::ACCEPTED,
        Err(report) if matches!(report.current_context(), DomainError::NotFound(_)) => {
            StatusCode::NOT_FOUND
        }
        Err(report) => {
            // An outage: the provider delivers again later.
            tracing::warn!(error = ?report, "a trigger's delivery could not be taken");
            StatusCode::SERVICE_UNAVAILABLE
        }
    }
}
