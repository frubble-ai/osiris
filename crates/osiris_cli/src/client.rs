//! Calling osiris with the profile's keys: the version header always sent, error answers turned
//! into their code and message.

use crate::profile::Profile;
use anyhow::{Context, bail};
use osiris_api_models::{
    errors::ErrorBody,
    version::{CURRENT, HEADER},
};
use reqwest::Method;
use serde_json::Value;

/// Which key a call needs.
#[derive(Clone, Copy)]
pub enum Key {
    Api,
    Admin,
}

pub struct Client {
    profile: Profile,
    http: reqwest::Client,
}

impl Client {
    pub fn new(profile: Profile) -> Self {
        Self {
            profile,
            http: reqwest::Client::new(),
        }
    }

    pub async fn call(
        &self,
        key: Key,
        method: Method,
        path: &str,
        body: Option<Value>,
        query: &[(&str, String)],
    ) -> anyhow::Result<Value> {
        let secret = match key {
            Key::Api => self
                .profile
                .api_key
                .as_deref()
                .context("no API key in the profile: `osiris login --api-key …`")?,
            Key::Admin => self
                .profile
                .admin_key
                .as_deref()
                .context("no admin key in the profile: `osiris login --admin-key …`")?,
        };
        let url = format!("{}{path}", self.profile.url.trim_end_matches('/'));
        let mut request = self
            .http
            .request(method, &url)
            .bearer_auth(secret)
            .header(HEADER, CURRENT)
            .query(query);
        if let Some(body) = body {
            request = request.json(&body);
        }
        let response = request
            .send()
            .await
            .with_context(|| format!("calling {url}"))?;
        let status = response.status();
        let text = response.text().await?;
        if !status.is_success() {
            match serde_json::from_str::<ErrorBody>(&text) {
                Ok(error) => bail!(
                    "{} {}: {}",
                    error.error.code,
                    error.error.error_type,
                    error.error.message
                ),
                Err(_) => bail!("{status}: {text}"),
            }
        }
        Ok(if text.is_empty() {
            Value::Null
        } else {
            serde_json::from_str(&text)?
        })
    }
}

/// Print an answer as indented JSON.
pub fn print(value: &Value) -> anyhow::Result<()> {
    println!("{}", serde_json::to_string_pretty(value)?);
    Ok(())
}
