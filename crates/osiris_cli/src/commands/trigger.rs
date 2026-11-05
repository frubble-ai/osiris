//! `osiris trigger connection.expired --connection ca_…`: a test event, the connection untouched.

use crate::{
    client::{Client, Key, print},
    profile,
};
use clap::Args;
use reqwest::Method;
use serde_json::json;

#[derive(Args)]
pub struct Trigger {
    /// `connection.active`, `connection.expired` or `connection.revoked`.
    event: String,
    #[arg(long)]
    connection: String,
}

impl Trigger {
    pub async fn run(self) -> anyhow::Result<()> {
        let body = json!({ "type": self.event, "connected_account_id": self.connection });
        print(
            &Client::new(profile::load()?)
                .call(
                    Key::Admin,
                    Method::POST,
                    "/v1/events/trigger",
                    Some(body),
                    &[],
                )
                .await?,
        )
    }
}
