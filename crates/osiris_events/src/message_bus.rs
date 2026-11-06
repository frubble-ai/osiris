//! The bus use cases send through: wraps a message in its envelope and publishes it on its
//! channel.

use crate::{Envelope, Message, Transport, TransportError};
use std::sync::Arc;

#[derive(Clone)]
pub struct MessageBus {
    transport: Arc<dyn Transport>,
}

impl MessageBus {
    pub fn new(transport: Arc<dyn Transport>) -> Self {
        Self { transport }
    }

    pub async fn send(&self, message: Message) -> Result<Envelope, TransportError> {
        let channel = message.channel();
        let envelope = Envelope::wrap(message);
        let data = serde_json::to_vec(&envelope).map_err(|error| TransportError {
            channel: channel.as_str(),
            message: error.to_string(),
        })?;
        self.transport
            .publish(channel, data, &[("type", envelope.message.message_type())])
            .await?;
        Ok(envelope)
    }
}
