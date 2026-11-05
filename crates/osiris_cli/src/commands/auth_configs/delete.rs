//! `osiris auth-configs delete ac_…`.

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
                    Key::Admin,
                    Method::DELETE,
                    &format!("/v1/auth-configs/{}", self.id),
                    None,
                    &[],
                )
                .await?,
        )
    }
}
