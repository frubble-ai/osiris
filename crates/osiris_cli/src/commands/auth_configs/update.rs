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
    pub async fn run(self) -> anyhow::Result<()> {
        let mut body = Map::new();
        if let Some(id) = self.client_id {
            body.insert("client_id".into(), Value::from(id));
        }
        if let Some(secret) = self.client_secret {
            body.insert("client_secret".into(), Value::from(secret));
        }
        if !self.scopes.is_empty() {
            body.insert("scopes".into(), Value::from(self.scopes));
        }
        let path = format!("/v1/auth-configs/{}", self.id);
        print(
            &Client::new(profile::load()?)
                .call(
                    Key::Admin,
                    Method::PATCH,
                    &path,
                    Some(Value::Object(body)),
                    &[],
                )
                .await?,
        )
    }
}
