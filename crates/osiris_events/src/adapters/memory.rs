//! Keeps what was published, for tests to read: each message's bytes and attributes as they were
//! sent, and its envelope when it reads as one.

use crate::{Channel, Envelope, Transport, TransportError};
use async_trait::async_trait;
use std::sync::{Mutex, PoisonError};

/// A message as it was handed to the transport.
#[derive(Debug, Clone)]
pub struct Published {
    pub channel: Channel,
    pub data: Vec<u8>,
    pub attributes: Vec<(String, String)>,
}

#[derive(Default)]
pub struct MemoryTransport {
    published: Mutex<Vec<Published>>,
    failing: Mutex<bool>,
}

impl MemoryTransport {
    /// Every envelope published so far, in order. A message written in an older release's shape
    /// may not read as the current envelope; `sent` has those.
    pub fn published(&self) -> Vec<(Channel, Envelope)> {
        self.sent()
            .into_iter()
            .filter_map(|sent| {
                serde_json::from_slice(&sent.data)
                    .ok()
                    .map(|envelope| (sent.channel, envelope))
            })
            .collect()
    }

    /// Every message published so far, in order, as it was sent.
    pub fn sent(&self) -> Vec<Published> {
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
        attributes: &[(&str, &str)],
    ) -> Result<(), TransportError> {
        if *self.failing.lock().unwrap_or_else(PoisonError::into_inner) {
            return Err(TransportError {
                channel: channel.as_str(),
                message: "unavailable".into(),
            });
        }
        self.published
            .lock()
            .unwrap_or_else(PoisonError::into_inner)
            .push(Published {
                channel,
                data,
                attributes: attributes
                    .iter()
                    .map(|(name, value)| ((*name).to_owned(), (*value).to_owned()))
                    .collect(),
            });
        Ok(())
    }
}
