//! The Pub/Sub transport held to the contract every transport is, against the emulator from
//! docker-compose (`PUBSUB_EMULATOR_HOST`, with the `osiris-events` topic `just up` creates).
//! Locally, without the emulator the test is skipped with a note; in CI (`CI` set) it must run.

#![allow(clippy::print_stderr, reason = "a skip is reported on stderr")]

use async_trait::async_trait;
use osiris_events::{
    Transport,
    testing::{TransportHarness, transport_contract},
};
use osiris_pubsub::{PubSubConfig, PubSubTransport};
use std::sync::Arc;

struct Emulator(Arc<PubSubTransport>);

#[async_trait]
impl TransportHarness for Emulator {
    fn transport(&self) -> Arc<dyn Transport> {
        self.0.clone()
    }
}

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
    transport_contract(&Emulator(Arc::new(transport))).await;
}
