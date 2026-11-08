//! `GET /v1/triggers`: triggers, of one connection or of all, in batches.

use crate::use_cases::cases::triggers::list_triggers::ListTriggersArgs;
use crate::{
    http::{auth::Auth, error::ApiResult, query_params::QueryParams, server_wrap, state::ApiState},
    routes::{cursor::cursor, list::list},
};
use axum::{
    Json,
    extract::State,
    http::{HeaderMap, StatusCode},
};
use osiris_api_models::{
    list::List,
    triggers::{Trigger, TriggerFilter},
};
use osiris_env::Flow;

/// Triggers, of one connection or of all, in batches.
#[utoipa::path(get, path = "/v1/triggers", tag = "triggers", params(TriggerFilter),
    responses((status = 200, body = List<Trigger>)), security(("api_key" = [])))]
pub async fn list_triggers(
    State(state): State<ApiState>,
    headers: HeaderMap,
    QueryParams(query): QueryParams<TriggerFilter>,
) -> ApiResult<(StatusCode, Json<List<Trigger>>)> {
    let cursor = cursor(&state, query.after, query.before, query.limit)?;
    server_wrap(
        Flow::TriggersList,
        &state,
        &headers,
        Auth::Api,
        StatusCode::OK,
        |use_cases| async move {
            use_cases
                .list_triggers(ListTriggersArgs {
                    connected_account_id: query.connected_account_id.as_deref(),
                    cursor: &cursor,
                })
                .await
                .map(list)
        },
    )
    .await
}
