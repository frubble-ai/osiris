//! The Pub/Sub transport, offered to the transport contract.

use crate::PubSubTransport;
use async_trait::async_trait;
use osiris_events::{Transport, testing::TransportHarness};
use std::sync::Arc;

pub struct MockEmulatorHarness(pub Arc<PubSubTransport>);

#[async_trait]
impl TransportHarness for MockEmulatorHarness {
    fn transport(&self) -> Arc<dyn Transport> {
        self.0.clone()
    }
}
