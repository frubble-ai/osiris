//! `osiris auth-configs create --provider github --client-id … --client-secret …`.

use crate::{
    client::{Client, Key, print},
    profile,
};
use clap::Args;
use reqwest::Method;
use serde_json::json;

#[derive(Args)]
pub struct Create {
    #[arg(long)]
    provider: String,
    #[arg(long)]
    client_id: String,
    /// Read from `OSIRIS_CLIENT_SECRET` when left out, so it stays out of the shell's history.
    #[arg(long, env = "OSIRIS_CLIENT_SECRET", hide_env_values = true)]
    client_secret: String,
    /// Repeat for each scope.
    #[arg(long = "scope")]
    scopes: Vec<String>,
}

impl Create {
    fn body(&self) -> serde_json::Value {
        json!({ "provider": self.provider, "client_id": self.client_id, "client_secret": self.client_secret, "scopes": self.scopes })
    }

    pub async fn run(self) -> anyhow::Result<()> {
        print(
            &Client::new(profile::load()?)
                .call(
                    Key::Admin,
                    Method::POST,
                    "/v1/auth-configs",
                    Some(self.body()),
                    &[],
                )
                .await?,
        )
    }
}

#[cfg(test)]
mod tests;
