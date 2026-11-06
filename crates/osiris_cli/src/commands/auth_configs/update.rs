//! `osiris auth-configs update ac_… [--client-id …] [--client-secret …] [--scope …]`.

use crate::{
    client::{Client, Key, print},
    profile,
};
use clap::Args;
use reqwest::Method;
use serde_json::{Map, Value};

#[derive(Args)]
pub struct Update {
    id: String,
    #[arg(long)]
    client_id: Option<String>,
    #[arg(long, env = "OSIRIS_CLIENT_SECRET", hide_env_values = true)]
    client_secret: Option<String>,
    /// Repeat for each scope; replaces them all.
    #[arg(long = "scope")]
    scopes: Vec<String>,
}

impl Update {
    fn path(&self) -> String {
        format!("/v1/auth-configs/{}", self.id)
    }

    /// Only what was given: a flag left out leaves the app's value as it is, and no `--scope` leaves
    /// its scopes alone instead of emptying them.
    fn body(&self) -> Value {
        let mut body = Map::new();
        if let Some(id) = &self.client_id {
            body.insert("client_id".into(), Value::from(id.clone()));
        }
        if let Some(secret) = &self.client_secret {
            body.insert("client_secret".into(), Value::from(secret.clone()));
        }
        if !self.scopes.is_empty() {
            body.insert("scopes".into(), Value::from(self.scopes.clone()));
        }
        Value::Object(body)
    }

    pub async fn run(self) -> anyhow::Result<()> {
        print(
            &Client::new(profile::load()?)
                .call(
                    Key::Admin,
                    Method::PATCH,
                    &self.path(),
                    Some(self.body()),
                    &[],
                )
                .await?,
        )
    }
}

#[cfg(test)]
mod tests;
