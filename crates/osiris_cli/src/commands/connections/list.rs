//! `osiris connections list [--user …] [--provider …] [--status …]`.

use crate::{
    client::{Client, Key, print},
    profile,
};
use clap::Args;
use reqwest::Method;

#[derive(Args)]
pub struct List {
    #[arg(long)]
    user: Option<String>,
    #[arg(long)]
    provider: Option<String>,
    #[arg(long)]
    status: Option<String>,
    #[arg(long)]
    after: Option<String>,
    #[arg(long)]
    limit: Option<i64>,
}

impl List {
    pub async fn run(self) -> anyhow::Result<()> {
        let query: Vec<(&str, String)> = [
            ("user_id", self.user),
            ("provider", self.provider),
            ("status", self.status),
            ("after", self.after),
            ("limit", self.limit.map(|limit| limit.to_string())),
        ]
        .into_iter()
        .filter_map(|(name, value)| value.map(|value| (name, value)))
        .collect();
        print(
            &Client::new(profile::load()?)
                .call(
                    Key::Api,
                    Method::GET,
                    "/v1/connected-accounts",
                    None,
                    &query,
                )
                .await?,
        )
    }
}
