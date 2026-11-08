//! `DELETE /v1/triggers/{id}`: stop watching; the subscription is removed at the provider.

use crate::{
    http::{auth::Auth, error::ApiResult, server_wrap, state::ApiState},
    use_cases::cases::delete_trigger::DeleteTriggerArgs,
};
use axum::{
    Json,
    extract::{Path, State},
    http::{HeaderMap, StatusCode},
};
use osiris_api_models::{auth_configs::Deleted, errors::ErrorBody};
use osiris_env::Flow;

/// Delete a trigger: its subscription is removed at the provider (a webhook, for example). A
/// provider that can't be reached is answered 502 and the trigger kept, to delete again. A
/// subscription the provider refuses to remove, or that the connection no longer reaches, is
/// left, and the trigger deleted all the same.
#[utoipa::path(delete, path = "/v1/triggers/{id}", tag = "triggers", params(("id" = String, Path)),
    params(("Idempotency-Key" = Option<String>, Header, description = "Runs the request once; a retry gets the first answer")),
    responses((status = 200, body = Deleted), (status = 404, body = ErrorBody), (status = 502, body = ErrorBody)), security(("api_key" = [])))]
pub async fn delete_trigger(
    State(state): State<ApiState>,
    headers: HeaderMap,
    Path(id): Path<String>,
) -> ApiResult<(StatusCode, Json<Deleted>)> {
    server_wrap(
        Flow::TriggersDelete,
        &state,
        &headers,
        Auth::Api,
        StatusCode::OK,
        |use_cases| async move {
            use_cases
                .delete_trigger(DeleteTriggerArgs { id: &id })
                .await?;
            Ok(Deleted { id, deleted: true })
        },
    )
    .await
}
