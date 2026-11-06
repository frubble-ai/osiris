//! `osiris auth-configs get ac_…`.

use crate::{
    client::{Client, Key, print},
    profile,
};
use clap::Args;
use reqwest::Method;

#[derive(Args)]
pub struct Get {
    id: String,
}

impl Get {
    fn path(&self) -> String {
        format!("/v1/auth-configs/{}", self.id)
    }

    pub async fn run(self) -> anyhow::Result<()> {
        print(
            &Client::new(profile::load()?)
                .call(Key::Admin, Method::GET, &self.path(), None, &[])
                .await?,
        )
    }
}

#[cfg(test)]
mod tests;
