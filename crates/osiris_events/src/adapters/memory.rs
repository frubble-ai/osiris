//! Keeps what was published, for tests to read.

use crate::{Channel, Envelope, Transport, TransportError};
use async_trait::async_trait;
use std::sync::{Mutex, PoisonError};

#[derive(Default)]
pub struct MemoryTransport {
    published: Mutex<Vec<(Channel, Envelope)>>,
    failing: Mutex<bool>,
}

impl MemoryTransport {
    /// Every envelope published so far, in order.
    pub fn published(&self) -> Vec<(Channel, Envelope)> {
        self.published
            .lock()
            .unwrap_or_else(PoisonError::into_inner)
            .clone()
    }

    /// Make every publish fail (or work again), as an unreachable Pub/Sub would.
    pub fn fail(&self, failing: bool) {
        *self.failing.lock().unwrap_or_else(PoisonError::into_inner) = failing;
    }
}

#[async_trait]
impl Transport for MemoryTransport {
    async fn publish(
        &self,
        channel: Channel,
        data: Vec<u8>,
        _: &[(&str, &str)],
    ) -> Result<(), TransportError> {
        if *self.failing.lock().unwrap_or_else(PoisonError::into_inner) {
            return Err(TransportError {
                channel: channel.as_str(),
                message: "unavailable".into(),
            });
        }
        let envelope = serde_json::from_slice(&data).map_err(|error| TransportError {
            channel: channel.as_str(),
            message: error.to_string(),
        })?;
        self.published
            .lock()
            .unwrap_or_else(PoisonError::into_inner)
            .push((channel, envelope));
        Ok(())
    }
}
