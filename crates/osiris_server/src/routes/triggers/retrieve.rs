//! `GET /v1/triggers/{id}`: one trigger.

use crate::{
    http::{auth::Auth, error::ApiResult, server_wrap, state::ApiState},
    use_cases::cases::triggers::get_trigger::GetTriggerArgs,
};
use axum::{
    Json,
    extract::{Path, State},
    http::{HeaderMap, StatusCode},
};
use osiris_api_models::{errors::ErrorBody, triggers::Trigger};
use osiris_env::Flow;

/// One trigger.
#[utoipa::path(get, path = "/v1/triggers/{id}", tag = "triggers", params(("id" = String, Path)),
    responses((status = 200, body = Trigger), (status = 404, body = ErrorBody)), security(("api_key" = [])))]
pub async fn retrieve_trigger(
    State(state): State<ApiState>,
    headers: HeaderMap,
    Path(id): Path<String>,
) -> ApiResult<(StatusCode, Json<Trigger>)> {
    server_wrap(
        Flow::TriggersRetrieve,
        &state,
        &headers,
        Auth::Api,
        StatusCode::OK,
        |use_cases| async move {
            use_cases
                .get_trigger(GetTriggerArgs { id: &id })
                .await
                .map(Trigger::from)
        },
    )
    .await
}
