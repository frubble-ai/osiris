//! The in-memory transport, offered to the transport contract; it can be made unable to publish.

use crate::{Transport, adapters::memory::MemoryTransport, testing::TransportHarness};
use async_trait::async_trait;
use std::sync::Arc;

pub struct MockMemoryHarness(pub Arc<MemoryTransport>);

#[async_trait]
impl TransportHarness for MockMemoryHarness {
    fn transport(&self) -> Arc<dyn Transport> {
        self.0.clone()
    }

    async fn make_unavailable(&self) -> bool {
        self.0.fail(true);
        true
    }
}
