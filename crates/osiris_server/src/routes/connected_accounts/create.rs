//! `POST /v1/connected-accounts`: start connecting a user.

use crate::{
    http::{auth::Auth, error::ApiResult, server_wrap, state::ApiState},
    use_cases::cases::start_connect::StartConnectArgs,
};
use axum::{
    Json,
    extract::State,
    http::{HeaderMap, StatusCode},
};
use osiris_api_models::{
    connected_accounts::{ConnectedAccountCreated, CreateConnectedAccount},
    errors::ErrorBody,
};
use osiris_env::Flow;

/// Start connecting a user: a pending connection and the provider's consent URL. After the
/// consent, osiris sends the browser to `callback_url`.
#[utoipa::path(post, path = "/v1/connected-accounts", tag = "connected-accounts", request_body = CreateConnectedAccount,
    params(("Idempotency-Key" = Option<String>, Header, description = "Runs the request once; a retry gets the first answer")),
    responses((status = 201, body = ConnectedAccountCreated), (status = 400, body = ErrorBody), (status = 404, body = ErrorBody)),
    security(("api_key" = [])))]
pub async fn create_connected_account(
    State(state): State<ApiState>,
    headers: HeaderMap,
    Json(body): Json<CreateConnectedAccount>,
) -> ApiResult<(StatusCode, Json<ConnectedAccountCreated>)> {
    let start = StartConnectArgs {
        auth_config_id: body.auth_config_id,
        user_id: body.user_id,
        alias: body.alias,
        callback_url: body.callback_url,
    };
    server_wrap(
        Flow::ConnectedAccountsCreate,
        &state,
        &headers,
        Auth::Api,
        StatusCode::CREATED,
        |use_cases| async move {
            let (account, redirect_url) = use_cases.start_connect(start).await?;
            Ok(ConnectedAccountCreated {
                id: account.id,
                status: account.status,
                redirect_url,
            })
        },
    )
    .await
}
