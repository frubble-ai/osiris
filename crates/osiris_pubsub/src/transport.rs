//! The `Transport` implementation for Pub/Sub. It builds one publisher for the events topic and
//! reuses it.

use crate::PubSubConfig;
use async_trait::async_trait;
use google_cloud_pubsub::{client::Publisher, model::Message};
use osiris_events::{Channel, Transport, TransportError};

pub struct PubSubTransport {
    events: Publisher,
}

impl PubSubTransport {
    pub async fn connect(config: &PubSubConfig) -> Result<Self, TransportError> {
        let mut builder = Publisher::builder(format!(
            "projects/{}/topics/{}",
            config.project, config.events_topic
        ));
        if let Some(host) = &config.emulator_host {
            builder = builder
                .with_endpoint(format!("http://{host}"))
                .with_credentials(
                    google_cloud_auth::credentials::anonymous::Builder::new().build(),
                );
        }
        let events = builder.build().await.map_err(|error| TransportError {
            channel: Channel::Events.as_str(),
            message: error.to_string(),
        })?;
        Ok(Self { events })
    }
}

#[async_trait]
impl Transport for PubSubTransport {
    async fn publish(
        &self,
        channel: Channel,
        data: Vec<u8>,
        attributes: &[(&str, &str)],
    ) -> Result<(), TransportError> {
        let publisher = match channel {
            Channel::Events => &self.events,
        };
        let message = Message::new().set_data(data).set_attributes(
            attributes
                .iter()
                .map(|(key, value)| ((*key).to_owned(), (*value).to_owned())),
        );
        publisher
            .publish(message)
            .await
            .map(|_| ())
            .map_err(|error| TransportError {
                channel: channel.as_str(),
                message: error.to_string(),
            })
    }
}
