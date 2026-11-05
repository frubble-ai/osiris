//! For development (`osiris listen`): a pull subscription of its own on the events topic, each
//! event `POST`ed to an endpoint as Pub/Sub would push it, until `stop` completes; the subscription
//! is removed afterwards.

use crate::PubSubConfig;
use base64::{Engine, engine::general_purpose::STANDARD};
use google_cloud_pubsub::client::{Subscriber, SubscriptionAdmin};
use serde_json::json;

/// One event forwarded: its type, and the endpoint's status (or why it couldn't be reached).
pub struct Forwarded {
    pub event_type: String,
    pub answer: Result<u16, String>,
}

pub struct Listen<'a> {
    pub pubsub: &'a PubSubConfig,
    /// How long Pub/Sub waits for each event to be acknowledged.
    pub ack_deadline_seconds: i32,
    /// A suffix that keeps this subscription apart from others.
    pub subscription_suffix: &'a str,
    pub forward_to: &'a str,
}

fn anonymous() -> google_cloud_auth::credentials::Credentials {
    google_cloud_auth::credentials::anonymous::Builder::new().build()
}

pub async fn listen(
    listen: Listen<'_>,
    stop: impl Future<Output = ()>,
    on_forwarded: impl Fn(Forwarded),
) -> anyhow::Result<()> {
    let config = listen.pubsub;
    let name = format!(
        "projects/{}/subscriptions/osiris-cli-{}",
        config.project, listen.subscription_suffix
    );
    let mut admin = SubscriptionAdmin::builder();
    let mut subscriber = Subscriber::builder();
    if let Some(host) = &config.emulator_host {
        admin = admin
            .with_endpoint(format!("http://{host}"))
            .with_credentials(anonymous());
        subscriber = subscriber
            .with_endpoint(format!("http://{host}"))
            .with_credentials(anonymous());
    }
    let admin = admin.build().await?;
    admin
        .create_subscription()
        .set_name(&name)
        .set_topic(format!(
            "projects/{}/topics/{}",
            config.project, config.events_topic
        ))
        .set_ack_deadline_seconds(listen.ack_deadline_seconds)
        .send()
        .await?;
    let subscriber = subscriber.build().await?;
    let forwarded = async {
        let http = reqwest::Client::new();
        let mut stream = subscriber.subscribe(&name).build();
        while let Some((message, handler)) = stream.next().await.transpose()? {
            let push = json!({
                "message": { "data": STANDARD.encode(&message.data), "messageId": message.message_id, "attributes": message.attributes },
                "subscription": name,
            });
            let event_type = message.attributes.get("type").cloned().unwrap_or_default();
            let answer = http
                .post(listen.forward_to)
                .json(&push)
                .send()
                .await
                .map(|response| response.status().as_u16())
                .map_err(|error| error.to_string());
            on_forwarded(Forwarded { event_type, answer });
            handler.ack();
        }
        anyhow::Ok(())
    };
    let result = tokio::select! {
        result = forwarded => result,
        () = stop => Ok(()),
    };
    admin
        .delete_subscription()
        .set_subscription(&name)
        .send()
        .await?;
    result
}
