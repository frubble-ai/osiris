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
    pub async fn run(self) -> anyhow::Result<()> {
        print(
            &Client::new(profile::load()?)
                .call(
                    Key::Api,
                    Method::DELETE,
                    &format!("/v1/connected-accounts/{}", self.id),
                    None,
                    &[],
                )
                .await?,
        )
    }
}
