//! The Pub/Sub transport held to the contract every transport is, against the emulator from
//! docker-compose (`PUBSUB_EMULATOR_HOST`, with the `osiris-events` topic `just up` creates).
//! Locally, without the emulator the test is skipped with a note; in CI (`CI` set) it must run.

#![allow(clippy::print_stderr, reason = "a skip is reported on stderr")]

use osiris_events::testing::transport_contract;
use osiris_pubsub::{
    PubSubConfig, PubSubTransport, mocks::mock_emulator_harness::MockEmulatorHarness,
};
use std::sync::Arc;

#[tokio::test]
async fn the_pubsub_transport_keeps_the_contract() {
    let Some(host) = std::env::var("PUBSUB_EMULATOR_HOST")
        .ok()
        .filter(|host| !host.is_empty())
    else {
        assert!(
            std::env::var_os("CI").is_none(),
            "PUBSUB_EMULATOR_HOST is not set in CI"
        );
        eprintln!("skipped: PUBSUB_EMULATOR_HOST is not set");
        return;
    };
    let transport = PubSubTransport::connect(&PubSubConfig {
        project: "osiris-local".into(),
        events_topic: "osiris-events".into(),
        emulator_host: Some(host),
    })
    .await
    .expect("the emulator answers");
    transport_contract(&MockEmulatorHarness(Arc::new(transport))).await;
}
