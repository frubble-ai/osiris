//! `GET /v1/connected-accounts`: connections, filtered, in batches.

use crate::use_cases::cases::list_connected_accounts::ListConnectedAccountsArgs;
use crate::{
    http::{
        auth::Auth,
        error::{ApiError, ApiResult},
        server_wrap,
        state::ApiState,
    },
    routes::{cursor::cursor, list::list},
};
use axum::{
    Json,
    extract::{Query, State},
    http::{HeaderMap, StatusCode},
};
use osiris_api_models::{
    connected_accounts::{ConnectedAccount, ConnectedAccountFilter},
    list::List,
};
use osiris_domain::ConnectionStatus;
use osiris_env::Flow;
use osiris_storage::connected_accounts::AccountFilter;

/// Connections, filtered, in batches; `alias` finds what an attempt made.
#[utoipa::path(get, path = "/v1/connected-accounts", tag = "connected-accounts", params(ConnectedAccountFilter),
    responses((status = 200, body = List<ConnectedAccount>)), security(("api_key" = [])))]
pub async fn list_connected_accounts(
    State(state): State<ApiState>,
    headers: HeaderMap,
    Query(query): Query<ConnectedAccountFilter>,
) -> ApiResult<(StatusCode, Json<List<ConnectedAccount>>)> {
    let status = match query.status.as_deref() {
        None => None,
        Some(value) => Some(
            ConnectionStatus::parse(value)
                .ok_or_else(|| ApiError::Invalid(format!("unknown status {value:?}")))?,
        ),
    };
    let filter = AccountFilter {
        user_id: query.user_id,
        provider: query.provider,
        auth_config_id: query.auth_config_id,
        alias: query.alias,
        status,
    };
    let cursor = cursor(&state, query.after, query.before, query.limit);
    server_wrap(
        Flow::ConnectedAccountsList,
        &state,
        &headers,
        Auth::Api,
        StatusCode::OK,
        |use_cases| async move {
            use_cases
                .list_connected_accounts(ListConnectedAccountsArgs {
                    filter: &filter,
                    cursor: &cursor,
                })
                .await
                .map(list)
        },
    )
    .await
}
