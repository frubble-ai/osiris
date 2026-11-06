//! Where events for clients are published: the Pub/Sub project and topic, and the emulator
//! locally. `build_use_case_context` builds the transport; the use cases only see the bus.

use crate::config::env_vars::{optional, required};
use osiris_pubsub::PubSubConfig;

pub fn events_config_from_env() -> anyhow::Result<PubSubConfig> {
    Ok(PubSubConfig {
        project: required("PUBSUB_PROJECT")?,
        events_topic: required("OSIRIS_EVENTS_TOPIC")?,
        emulator_host: optional("PUBSUB_EMULATOR_HOST"),
    })
}
