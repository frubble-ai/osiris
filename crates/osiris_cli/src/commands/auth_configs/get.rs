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
    pub async fn run(self) -> anyhow::Result<()> {
        print(
            &Client::new(profile::load()?)
                .call(
                    Key::Admin,
                    Method::GET,
                    &format!("/v1/auth-configs/{}", self.id),
                    None,
                    &[],
                )
                .await?,
        )
    }
}
