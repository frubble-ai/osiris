//! `osiris listen --forward-to http://localhost:8081/push/osiris-events`: each event osiris
//! publishes, `POST`ed to the endpoint as Pub/Sub would push it, until Ctrl-C. The emulator when
//! the profile names one; else your Application Default Credentials.

use crate::profile;
use anyhow::Context;
use clap::Args;
use osiris_pubsub::{
    PubSubConfig,
    listen::{Forwarded, Listen as ListenArgs, listen},
};

#[derive(Args)]
pub struct Listen {
    /// Where to POST each event.
    #[arg(long)]
    forward_to: String,
    /// How long Pub/Sub waits for each event to be acknowledged.
    #[arg(long, default_value_t = 30)]
    ack_deadline_seconds: i32,
}

impl Listen {
    pub async fn run(self) -> anyhow::Result<()> {
        let profile = profile::load()?;
        let pubsub = PubSubConfig {
            project: profile
                .pubsub_project
                .clone()
                .context("no pubsub_project in the profile: `osiris login --pubsub-project …`")?,
            events_topic: profile
                .events_topic
                .clone()
                .context("no events_topic in the profile: `osiris login --events-topic …`")?,
            emulator_host: profile.pubsub_emulator_host.clone(),
        };
        println!(
            "listening on {}, forwarding to {} (Ctrl-C to stop)",
            pubsub.events_topic, self.forward_to
        );
        let suffix = osiris_common::ids::random(8).to_lowercase();
        listen(
            ListenArgs {
                pubsub: &pubsub,
                ack_deadline_seconds: self.ack_deadline_seconds,
                subscription_suffix: &suffix,
                forward_to: &self.forward_to,
            },
            async {
                let _ = tokio::signal::ctrl_c().await;
            },
            |Forwarded { event_type, answer }| match answer {
                Ok(status) => println!("{event_type} → {status}"),
                Err(error) => println!("{event_type} → {error}"),
            },
        )
        .await
    }
}
