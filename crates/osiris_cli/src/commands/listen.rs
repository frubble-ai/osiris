//! `osiris listen --forward-to http://localhost:8081/push/osiris-events`: a pull subscription of
//! its own on the events topic, each event `POSTed` to the endpoint as Pub/Sub would push it, and the
//! subscription removed on Ctrl-C. The emulator when the profile names one; else your Application
//! Default Credentials.

use crate::profile::{self, Profile};
use anyhow::Context;
use base64::{Engine, engine::general_purpose::STANDARD};
use clap::Args;
use google_cloud_pubsub::client::{Subscriber, SubscriptionAdmin};
use serde_json::json;

#[derive(Args)]
pub struct Listen {
    /// Where to POST each event.
    #[arg(long)]
    forward_to: String,
}

fn anonymous() -> google_cloud_auth::credentials::Credentials {
    google_cloud_auth::credentials::anonymous::Builder::new().build()
}

async fn admin(profile: &Profile) -> anyhow::Result<SubscriptionAdmin> {
    let mut builder = SubscriptionAdmin::builder();
    if let Some(host) = &profile.pubsub_emulator_host {
        builder = builder
            .with_endpoint(format!("http://{host}"))
            .with_credentials(anonymous());
    }
    Ok(builder.build().await?)
}

async fn subscriber(profile: &Profile) -> anyhow::Result<Subscriber> {
    let mut builder = Subscriber::builder();
    if let Some(host) = &profile.pubsub_emulator_host {
        builder = builder
            .with_endpoint(format!("http://{host}"))
            .with_credentials(anonymous());
    }
    Ok(builder.build().await?)
}

impl Listen {
    pub async fn run(self) -> anyhow::Result<()> {
        let profile = profile::load()?;
        let project = profile
            .pubsub_project
            .clone()
            .context("no pubsub_project in the profile: `osiris login --pubsub-project …`")?;
        let topic = profile
            .events_topic
            .clone()
            .context("no events_topic in the profile: `osiris login --events-topic …`")?;
        let name = format!(
            "projects/{project}/subscriptions/osiris-cli-{}",
            osiris_common::ids::random(8).to_lowercase()
        );
        let admin = admin(&profile).await?;
        admin
            .create_subscription()
            .set_name(&name)
            .set_topic(format!("projects/{project}/topics/{topic}"))
            .set_ack_deadline_seconds(30)
            .send()
            .await
            .context("creating the CLI's subscription")?;
        println!(
            "listening on {topic}, forwarding to {} (Ctrl-C to stop)",
            self.forward_to
        );
        let result = tokio::select! {
            result = forward(&profile, &name, &self.forward_to) => result,
            _ = tokio::signal::ctrl_c() => Ok(()),
        };
        admin
            .delete_subscription()
            .set_subscription(&name)
            .send()
            .await
            .context("removing the CLI's subscription")?;
        result
    }
}

async fn forward(profile: &Profile, subscription: &str, endpoint: &str) -> anyhow::Result<()> {
    let http = reqwest::Client::new();
    let mut stream = subscriber(profile).await?.subscribe(subscription).build();
    while let Some((message, handler)) = stream.next().await.transpose()? {
        let push = json!({
            "message": { "data": STANDARD.encode(&message.data), "messageId": message.message_id, "attributes": message.attributes },
            "subscription": subscription,
        });
        let event_type = message.attributes.get("type").cloned().unwrap_or_default();
        match http.post(endpoint).json(&push).send().await {
            Ok(response) => println!("{event_type} → {}", response.status()),
            Err(error) => println!("{event_type} → {error}"),
        }
        handler.ack();
    }
    Ok(())
}
