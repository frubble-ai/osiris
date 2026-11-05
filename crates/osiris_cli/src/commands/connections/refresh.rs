//! `osiris connections refresh ca_…`: refresh a connection's tokens now (admin key).

use crate::{
    client::{Client, Key, print},
    profile,
};
use clap::Args;
use reqwest::Method;

#[derive(Args)]
pub struct Refresh {
    id: String,
}

impl Refresh {
    pub async fn run(self) -> anyhow::Result<()> {
        print(
            &Client::new(profile::load()?)
                .call(
                    Key::Admin,
                    Method::POST,
                    &format!("/v1/connected-accounts/{}/refresh", self.id),
                    None,
                    &[],
                )
                .await?,
        )
    }
}
