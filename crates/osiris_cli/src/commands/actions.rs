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

impl Run {
    fn path(&self) -> String {
        format!(
            "/v1/connected-accounts/{}/actions/{}",
            self.connection, self.action
        )
    }
}

impl Actions {
    pub async fn run(self) -> anyhow::Result<()> {
        let Self::Run(run) = self;
        print(
            &Client::new(profile::load()?)
                .call(Key::Api, Method::POST, &run.path(), None, &[])
                .await?,
        )
    }
}

#[cfg(test)]
mod tests;
