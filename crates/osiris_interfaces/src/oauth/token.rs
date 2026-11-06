//! The token endpoint: exchanging a code, refreshing, and reading what it answers. Ported from
//! `OpenConnector` v1.9.4 (Apache-2.0), `src/oauth/oauth-token.ts`.

use super::definition::{ClientAuth, OAuth2Definition};
use crate::{error::ProviderError, oauth_client::OAuthClient, tokens::Tokens};
use osiris_common::secret::secret;
use serde_json::{Map, Value};
use url::Url;

/// Exchange the consent's `code` for tokens.
pub async fn exchange_code(
    provider: &'static str,
    http: &reqwest::Client,
    definition: &OAuth2Definition,
    client: &OAuthClient<'_>,
    code: &str,
    pkce_verifier: Option<&str>,
) -> Result<Tokens, ProviderError> {
    let mut form = vec![
        ("grant_type", "authorization_code"),
        ("code", code),
        ("redirect_uri", client.redirect_uri),
    ];
    if let (true, Some(verifier)) = (definition.pkce, pkce_verifier) {
        form.push(("code_verifier", verifier));
    }
    token_request(
        provider,
        http,
        definition,
        &definition.token_url,
        client,
        form,
    )
    .await
}

/// Trade a refresh token for fresh tokens.
pub async fn refresh(
    provider: &'static str,
    http: &reqwest::Client,
    definition: &OAuth2Definition,
    client: &OAuthClient<'_>,
    refresh_token: &str,
) -> Result<Tokens, ProviderError> {
    let url = definition
        .refresh_token_url
        .as_ref()
        .unwrap_or(&definition.token_url);
    let form = vec![
        ("grant_type", "refresh_token"),
        ("refresh_token", refresh_token),
    ];
    token_request(provider, http, definition, url, client, form).await
}

async fn token_request(
    provider: &'static str,
    http: &reqwest::Client,
    definition: &OAuth2Definition,
    url: &Url,
    client: &OAuthClient<'_>,
    mut form: Vec<(&str, &str)>,
) -> Result<Tokens, ProviderError> {
    let mut request = http
        .post(url.clone())
        .header(reqwest::header::ACCEPT, "application/json");
    match definition.client_auth {
        ClientAuth::Basic => {
            request = request.basic_auth(client.client_id, Some(client.client_secret));
        }
        ClientAuth::Post => {
            form.push(("client_id", client.client_id));
            form.push(("client_secret", client.client_secret));
        }
    }
    let response = request
        .form(&form)
        .send()
        .await
        .map_err(|error| ProviderError::unavailable(provider, error))?;
    let status = response.status();
    let body = response
        .text()
        .await
        .map_err(|error| ProviderError::unavailable(provider, error))?;
    parse_token_answer(provider, status, &body)
}

/// Error codes that mean "try again", not "this grant is dead": the standard ones (RFC 6749,
/// section 4.1.2.1) and the ones Slack answers with `ok: false`.
const TRANSIENT_ERRORS: &[&str] = &[
    "temporarily_unavailable",
    "server_error",
    "ratelimited",
    "service_unavailable",
    "internal_error",
    "fatal_error",
    "request_timeout",
];

/// A token answer. Some providers refuse with a 200 and an `error` (GitHub), or `ok: false`
/// (Slack); a token endpoint may echo credentials, so an unreadable body is never repeated.
fn parse_token_answer(
    provider: &'static str,
    status: reqwest::StatusCode,
    body: &str,
) -> Result<Tokens, ProviderError> {
    // A server error, a rate limit or a timeout is temporary, whatever the body says: retrying may
    // work, so the connection must not be ended for it. Any other refusal is permanent.
    if status.is_server_error()
        || status == reqwest::StatusCode::TOO_MANY_REQUESTS
        || status == reqwest::StatusCode::REQUEST_TIMEOUT
    {
        return Err(ProviderError::unavailable(
            provider,
            format!("token endpoint answered {status}"),
        ));
    }
    let Ok(Value::Object(answer)) = serde_json::from_str::<Value>(body) else {
        let what = if body.is_empty() {
            "an empty body"
        } else {
            "a body that is not JSON"
        };
        return Err(if status.is_success() {
            ProviderError::unavailable(
                provider,
                format!("token endpoint answered {status} with {what}"),
            )
        } else {
            ProviderError::refused(
                provider,
                format!("token endpoint answered {status} with {what}"),
            )
        });
    };
    let text = |field: &str| answer.get(field).and_then(Value::as_str).map(str::to_owned);
    let refused = answer.contains_key("error") || answer.get("ok") == Some(&Value::Bool(false));
    if refused || !status.is_success() {
        let message = text("error_description")
            .or_else(|| text("error"))
            .unwrap_or_else(|| format!("token endpoint answered {status}"));
        // An error code that says the failure is temporary does not end the grant, whatever the
        // HTTP status around it.
        if text("error").is_some_and(|code| TRANSIENT_ERRORS.contains(&code.as_str())) {
            return Err(ProviderError::unavailable(provider, message));
        }
        return Err(ProviderError::refused(provider, message));
    }
    let access_token = text("access_token")
        .or_else(|| text("token"))
        .ok_or_else(|| {
            ProviderError::unavailable(provider, "the token answer has no access_token")
        })?;
    Ok(Tokens {
        access_token: secret(access_token),
        refresh_token: text("refresh_token").map(secret),
        expires_in: expires_in(&answer),
        scope: text("scope").filter(|scope| !scope.is_empty()),
    })
}

/// `expires_in` as a JSON number or numeric string, positive and under ten years.
fn expires_in(answer: &Map<String, Value>) -> Option<i64> {
    let seconds = match answer.get("expires_in")? {
        Value::Number(number) => number.as_i64()?,
        Value::String(text) => text.trim().parse().ok()?,
        _ => return None,
    };
    (1..=315_360_000).contains(&seconds).then_some(seconds)
}

#[cfg(test)]
mod tests;
