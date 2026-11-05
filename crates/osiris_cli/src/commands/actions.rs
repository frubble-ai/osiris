//! `osiris actions run github.list_repositories --connection ca_…`.

use crate::{
    client::{Client, Key, print},
    profile,
};
use clap::{Args, Subcommand};
use reqwest::Method;

#[derive(Subcommand)]
pub enum Actions {
    /// Run an action and print its typed answer.
    Run(Run),
}

#[derive(Args)]
pub struct Run {
    /// e.g. `github.list_repositories`, `linear.list_teams`, `slack.list_channels`.
    action: String,
    #[arg(long)]
    connection: String,
}

impl Actions {
    pub async fn run(self) -> anyhow::Result<()> {
        let Self::Run(run) = self;
        let path = format!(
            "/v1/connected-accounts/{}/actions/{}",
            run.connection, run.action
        );
        print(
            &Client::new(profile::load()?)
                .call(Key::Api, Method::POST, &path, None, &[])
                .await?,
        )
    }
}
