//! `osiris connections delete ca_…`.

use crate::{
    client::{Client, Key, print},
    profile,
};
use clap::Args;
use reqwest::Method;

#[derive(Args)]
pub struct Delete {
    id: String,
}

impl Delete {
    fn path(&self) -> String {
        format!("/v1/connected-accounts/{}", self.id)
    }

    pub async fn run(self) -> anyhow::Result<()> {
        print(
            &Client::new(profile::load()?)
                .call(Key::Api, Method::DELETE, &self.path(), None, &[])
                .await?,
        )
    }
}

#[cfg(test)]
mod tests;
