//! The port adapters implement: hand bytes to a channel's topic.

use crate::Channel;
use async_trait::async_trait;

#[derive(Debug, thiserror::Error)]
#[error("publishing to {channel} failed: {message}")]
pub struct TransportError {
    pub channel: &'static str,
    pub message: String,
}

#[async_trait]
pub trait Transport: Send + Sync {
    /// Publish `data` with its `attributes` to `channel`; answers once the topic holds it.
    async fn publish(
        &self,
        channel: Channel,
        data: Vec<u8>,
        attributes: &[(&str, &str)],
    ) -> Result<(), TransportError>;
}
