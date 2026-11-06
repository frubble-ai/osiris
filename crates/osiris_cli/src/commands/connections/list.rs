//! `osiris connections list [--user …] [--provider …] [--status …]`.

use crate::{
    client::{Client, Key, present_params::present_params, print},
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
    fn query(&self) -> Vec<(&'static str, String)> {
        present_params([
            ("user_id", self.user.clone()),
            ("provider", self.provider.clone()),
            ("status", self.status.clone()),
            ("after", self.after.clone()),
            ("limit", self.limit.map(|limit| limit.to_string())),
        ])
    }

    pub async fn run(self) -> anyhow::Result<()> {
        print(
            &Client::new(profile::load()?)
                .call(
                    Key::Api,
                    Method::GET,
                    "/v1/connected-accounts",
                    None,
                    &self.query(),
                )
                .await?,
        )
    }
}

#[cfg(test)]
mod tests;
