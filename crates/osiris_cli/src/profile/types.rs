//! The profile: where osiris is, the keys to call it with, and where `listen` subscribes.

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct Profile {
    /// Osiris's origin, e.g. `http://localhost:8090`.
    pub url: String,
    pub api_key: Option<String>,
    pub admin_key: Option<String>,
    /// For `listen`: the Pub/Sub project and events topic, and the emulator locally.
    pub pubsub_project: Option<String>,
    pub events_topic: Option<String>,
    pub pubsub_emulator_host: Option<String>,
}
