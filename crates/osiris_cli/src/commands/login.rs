//! `osiris login`: save the profile. Flags left out keep what the profile had.

use crate::profile::{self, Profile};
use clap::Args;

/// Where a new profile points when `--url` is left out.
const DEFAULT_URL: &str = "http://localhost:8090";

#[derive(Args)]
pub struct Login {
    /// Osiris's origin. Left out, the saved one is kept, or `http://localhost:8090` for a new
    /// profile.
    #[arg(long)]
    url: Option<String>,
    #[arg(long, env = "OSIRIS_API_KEY", hide_env_values = true)]
    api_key: Option<String>,
    #[arg(long, env = "OSIRIS_ADMIN_KEY", hide_env_values = true)]
    admin_key: Option<String>,
    #[arg(long, env = "PUBSUB_PROJECT")]
    pubsub_project: Option<String>,
    #[arg(long, env = "OSIRIS_EVENTS_TOPIC")]
    events_topic: Option<String>,
    #[arg(long, env = "PUBSUB_EMULATOR_HOST")]
    pubsub_emulator_host: Option<String>,
}

impl Login {
    pub fn run(self) -> anyhow::Result<()> {
        // No profile yet starts an empty one; a profile that can't be read is an error, never
        // silently replaced.
        let old = profile::load_if_present()?;
        let path = profile::save(&self.merged_into(old))?;
        println!("profile saved to {}", path.display());
        Ok(())
    }

    /// The profile to save: each flag given replaces the saved value, the others are kept, and a
    /// new profile with no `--url` points at [`DEFAULT_URL`].
    fn merged_into(self, old: Option<Profile>) -> Profile {
        let old = old.unwrap_or_default();
        let url = self
            .url
            .or_else(|| Some(old.url.clone()).filter(|url| !url.is_empty()))
            .unwrap_or_else(|| DEFAULT_URL.to_owned());
        Profile {
            url,
            api_key: self.api_key.or(old.api_key),
            admin_key: self.admin_key.or(old.admin_key),
            pubsub_project: self.pubsub_project.or(old.pubsub_project),
            events_topic: self.events_topic.or(old.events_topic),
            pubsub_emulator_host: self.pubsub_emulator_host.or(old.pubsub_emulator_host),
        }
    }
}

#[cfg(test)]
mod tests;
