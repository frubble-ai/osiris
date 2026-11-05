//! `osiris connect github --user org_1`: start a consent on the provider's app and open it in the
//! browser. With one app for the provider it is chosen; with several, name one with `--auth-config`.

use crate::{
    client::{Client, Key},
    profile,
};
use anyhow::{Context, bail};
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
            let matching: Vec<String> = configs
                .get("data")
                .and_then(serde_json::Value::as_array)
                .into_iter()
                .flatten()
                .filter(|config| {
                    config.get("provider").and_then(serde_json::Value::as_str)
                        == Some(self.provider.as_str())
                })
                .filter_map(|config| {
                    config
                        .get("id")
                        .and_then(serde_json::Value::as_str)
                        .map(str::to_owned)
                })
                .collect();
            match matching.as_slice() {
                [one] => one.clone(),
                [] => bail!(
                    "no auth config for {0}: `osiris auth-configs create --provider {0} …`",
                    self.provider
                ),
                _ => bail!(
                    "several auth configs for {}: name one with --auth-config ({})",
                    self.provider,
                    matching.join(", ")
                ),
            }
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
