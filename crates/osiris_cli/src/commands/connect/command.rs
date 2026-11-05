//! Starts a consent and opens it in the browser.

use crate::{
    client::{Client, Key},
    profile,
};

use super::choose_auth_config::choose_auth_config;
use anyhow::Context;
use clap::Args;
use reqwest::Method;
use serde_json::json;

#[derive(Args)]
pub struct Connect {
    /// `github`, `linear` or `slack`.
    provider: String,
    /// Who the connection is for (your own id for them).
    #[arg(long)]
    user: String,
    #[arg(long)]
    auth_config: Option<String>,
    /// Where the browser goes after the consent.
    #[arg(long, default_value = "http://localhost:8090/health")]
    callback_url: String,
    /// Print the consent URL without opening it.
    #[arg(long)]
    no_browser: bool,
}

impl Connect {
    pub async fn run(self) -> anyhow::Result<()> {
        let client = Client::new(profile::load()?);
        let auth_config = if let Some(id) = self.auth_config {
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
            choose_auth_config(&self.provider, &configs)?
        };
        let body = json!({ "auth_config_id": auth_config, "user_id": self.user, "callback_url": self.callback_url });
        let created = client
            .call(
                Key::Api,
                Method::POST,
                "/v1/connected-accounts",
                Some(body),
                &[],
            )
            .await?;
        let url = created
            .get("redirect_url")
            .and_then(serde_json::Value::as_str)
            .context("osiris answered no redirect_url")?;
        println!(
            "connection {} is pending; consent at:\n{url}",
            created
                .get("id")
                .and_then(serde_json::Value::as_str)
                .unwrap_or("?")
        );
        if !self.no_browser {
            let opener = if cfg!(target_os = "macos") {
                "open"
            } else {
                "xdg-open"
            };
            if std::process::Command::new(opener)
                .arg(url)
                .status()
                .is_err()
            {
                println!("(could not open a browser; open the URL yourself)");
            }
        }
        Ok(())
    }
}
