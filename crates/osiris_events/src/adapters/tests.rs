//! The in-memory transport, held to the contract every transport is.

use super::{memory::MemoryTransport, mocks::mock_memory_harness::MockMemoryHarness};
use crate::testing::transport_contract;
use std::sync::Arc;

#[tokio::test]
async fn the_memory_transport_keeps_the_contract() {
    transport_contract(&MockMemoryHarness(Arc::new(MemoryTransport::default()))).await;
}
