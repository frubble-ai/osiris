//! The in-memory transport, held to the contract every transport is.

use super::memory::MemoryTransport;
use crate::{
    Transport,
    testing::{TransportHarness, transport_contract},
};
use async_trait::async_trait;
use std::sync::Arc;

struct Memory(Arc<MemoryTransport>);

#[async_trait]
impl TransportHarness for Memory {
    fn transport(&self) -> Arc<dyn Transport> {
        self.0.clone()
    }

    async fn make_unavailable(&self) -> bool {
        self.0.fail(true);
        true
    }
}

#[tokio::test]
async fn the_memory_transport_keeps_the_contract() {
    transport_contract(&Memory(Arc::new(MemoryTransport::default()))).await;
}
