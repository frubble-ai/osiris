//! `POST /v1/connected-accounts`: start connecting a user through their consent, or connect as an
//! app's installation.

use crate::{
    http::{auth::Auth, error::ApiResult, json_body::JsonBody, server_wrap, state::ApiState},
    use_cases::cases::{
        connect_installation::ConnectInstallationArgs, start_connect::StartConnectArgs,
    },
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
use osiris_domain::DomainError;
use osiris_env::Flow;

/// Start connecting a user: a pending connection and the provider's consent URL. After the
/// consent, osiris sends the browser to `callback_url`. With `installation_id`, connect as that
/// installation of the auth config's app instead: active at once, no consent.
#[utoipa::path(post, path = "/v1/connected-accounts", tag = "connected-accounts", request_body = CreateConnectedAccount,
    params(("Idempotency-Key" = Option<String>, Header, description = "Runs the request once; a retry gets the first answer")),
    responses((status = 201, body = ConnectedAccountCreated), (status = 400, body = ErrorBody), (status = 404, body = ErrorBody), (status = 424, body = ErrorBody), (status = 502, body = ErrorBody)),
    security(("api_key" = [])))]
pub async fn create_connected_account(
    State(state): State<ApiState>,
    headers: HeaderMap,
    JsonBody(body): JsonBody<CreateConnectedAccount>,
) -> ApiResult<(StatusCode, Json<ConnectedAccountCreated>)> {
    server_wrap(
        Flow::ConnectedAccountsCreate,
        &state,
        &headers,
        Auth::Api,
        StatusCode::CREATED,
        |use_cases| async move {
            if let Some(installation_id) = body.installation_id {
                let account = use_cases
                    .connect_installation(ConnectInstallationArgs {
                        auth_config_id: body.auth_config_id,
                        user_id: body.user_id,
                        alias: body.alias,
                        installation_id,
                    })
                    .await?;
                return Ok(ConnectedAccountCreated {
                    id: account.id,
                    status: account.status,
                    redirect_url: None,
                });
            }
            let callback_url = body.callback_url.ok_or_else(|| {
                DomainError::Invalid("callback_url is required for a consent".into())
            })?;
            let (account, redirect_url) = use_cases
                .start_connect(StartConnectArgs {
                    auth_config_id: body.auth_config_id,
                    user_id: body.user_id,
                    alias: body.alias,
                    callback_url,
                })
                .await?;
            Ok(ConnectedAccountCreated {
                id: account.id,
                status: account.status,
                redirect_url: Some(redirect_url),
            })
        },
    )
    .await
}
