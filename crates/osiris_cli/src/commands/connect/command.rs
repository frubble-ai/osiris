//! Starts a consent and opens it in the browser.

use crate::{client::Client, profile};

use super::{default_callback_url::default_callback_url, start_consent::start_consent};
use clap::Args;

#[derive(Args)]
pub struct Connect {
    /// The provider's id, as its auth configs name it.
    provider: String,
    /// Who the connection is for (your own id for them).
    #[arg(long)]
    user: String,
    #[arg(long)]
    auth_config: Option<String>,
    /// Where the browser goes after the consent. Left out, it is the profile's osiris `/health`.
    #[arg(long)]
    callback_url: Option<String>,
    /// Print the consent URL without opening it.
    #[arg(long)]
    no_browser: bool,
}

impl Connect {
    pub async fn run(self) -> anyhow::Result<()> {
        let profile = profile::load()?;
        let callback_url = self
            .callback_url
            .unwrap_or_else(|| default_callback_url(&profile.url));
        let client = Client::new(profile);
        let started = start_consent(
            &client,
            &self.provider,
            &self.user,
            self.auth_config,
            &callback_url,
        )
        .await?;
        let url = &started.consent_url;
        println!("connection {} is pending; consent at:\n{url}", started.id);
        if !self.no_browser {
            let opener = if cfg!(target_os = "macos") {
                "open"
            } else {
                "xdg-open"
            };
            if std::process::Command::new(opener)
                .arg(url)
                .status()
                .is_err()
            {
                println!("(could not open a browser; open the URL yourself)");
            }
        }
        Ok(())
    }
}
