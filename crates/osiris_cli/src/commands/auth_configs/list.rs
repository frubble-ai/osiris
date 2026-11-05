//! `osiris auth-configs list`.

use crate::{
    client::{Client, Key, print},
    profile,
};
use clap::Args;
use reqwest::Method;

#[derive(Args)]
pub struct List {
    #[arg(long)]
    after: Option<String>,
    #[arg(long)]
    limit: Option<i64>,
}

impl List {
    pub async fn run(self) -> anyhow::Result<()> {
        let query: Vec<(&str, String)> = [
            ("after", self.after),
            ("limit", self.limit.map(|limit| limit.to_string())),
        ]
        .into_iter()
        .filter_map(|(name, value)| value.map(|value| (name, value)))
        .collect();
        print(
            &Client::new(profile::load()?)
                .call(Key::Admin, Method::GET, "/v1/auth-configs", None, &query)
                .await?,
        )
    }
}
