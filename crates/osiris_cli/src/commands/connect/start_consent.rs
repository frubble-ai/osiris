//! Opens a consent on osiris: finds the provider's app when none is named, and asks for the
//! connection.

use super::choose_auth_config::choose_auth_config;
use crate::client::{Client, Key};
use anyhow::Context;
use reqwest::Method;
use serde_json::{Value, json};

/// A connection waiting for its consent.
pub struct Started {
    pub id: String,
    pub consent_url: String,
}

pub async fn start_consent(
    client: &Client,
    provider: &str,
    user: &str,
    auth_config: Option<String>,
    callback_url: &str,
) -> anyhow::Result<Started> {
    let auth_config = if let Some(id) = auth_config {
        id
    } else {
        let configs = client
            .call(
                Key::Admin,
                Method::GET,
                "/v1/auth-configs",
                None,
                &[("limit", "100".to_owned())],
            )
            .await?;
        choose_auth_config(provider, &configs)?
    };
    let body =
        json!({ "auth_config_id": auth_config, "user_id": user, "callback_url": callback_url });
    let created = client
        .call(
            Key::Api,
            Method::POST,
            "/v1/connected-accounts",
            Some(body),
            &[],
        )
        .await?;
    let text = |field: &str| {
        created
            .get(field)
            .and_then(Value::as_str)
            .map(str::to_owned)
    };
    Ok(Started {
        id: text("id").unwrap_or_else(|| "?".to_owned()),
        consent_url: text("redirect_url").context("osiris answered no redirect_url")?,
    })
}

#[cfg(test)]
mod tests;
