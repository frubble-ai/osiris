//! The bus use cases send through: wraps a message in its envelope, writes it in the API release
//! subscribers are pinned to, and publishes it on its channel. Messages are made in the current
//! release's shape and taken back through the version changes made since, as the API's answers
//! are, so a client pinned to an older release reads its events in that release's shape too.

use crate::{Envelope, Message, Transport, TransportError};
use osiris_api_models::version::{CURRENT, RELEASES, Release, translate::event_down};
use std::sync::Arc;

#[derive(Clone)]
pub struct MessageBus {
    transport: Arc<dyn Transport>,
    releases: &'static [Release],
    release: &'static Release,
}

impl MessageBus {
    /// A bus writing messages in the current release.
    pub fn new(transport: Arc<dyn Transport>) -> Self {
        Self {
            transport,
            releases: RELEASES,
            release: CURRENT,
        }
    }

    /// The same bus, writing messages in `release`, one of `releases`.
    #[must_use]
    pub fn written_in(mut self, releases: &'static [Release], release: &'static Release) -> Self {
        self.releases = releases;
        self.release = release;
        self
    }

    /// Sends `message`; answers its envelope as the current release writes it.
    pub async fn send(&self, message: Message) -> Result<Envelope, TransportError> {
        let channel = message.channel();
        let envelope = Envelope::wrap(message, self.release.name);
        let failed = |error: serde_json::Error| TransportError {
            channel: channel.as_str(),
            message: error.to_string(),
        };
        let mut json = serde_json::to_value(&envelope).map_err(failed)?;
        event_down(self.releases, self.release, &mut json);
        let data = serde_json::to_vec(&json).map_err(failed)?;
        self.transport
            .publish(
                channel,
                data,
                &[
                    ("type", envelope.message.message_type()),
                    ("osiris_version", self.release.name),
                ],
            )
            .await?;
        Ok(envelope)
    }
}
