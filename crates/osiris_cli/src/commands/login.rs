//! `osiris login`: save the profile. Flags left out keep what the profile had.

use crate::profile::{self, Profile};
use clap::Args;

#[derive(Args)]
pub struct Login {
    /// Osiris's origin.
    #[arg(long, default_value = "http://localhost:8090")]
    url: String,
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
        let old = profile::load().unwrap_or_default();
        let path = profile::save(&Profile {
            url: self.url,
            api_key: self.api_key.or(old.api_key),
            admin_key: self.admin_key.or(old.admin_key),
            pubsub_project: self.pubsub_project.or(old.pubsub_project),
            events_topic: self.events_topic.or(old.events_topic),
            pubsub_emulator_host: self.pubsub_emulator_host.or(old.pubsub_emulator_host),
        })?;
        println!("profile saved to {}", path.display());
        Ok(())
    }
}
