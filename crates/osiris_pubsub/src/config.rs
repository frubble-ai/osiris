//! The project and topic events are published to.

#[derive(Debug, Clone)]
pub struct PubSubConfig {
    pub project: String,
    pub events_topic: String,
    /// `host:port` of the emulator; none for the real Pub/Sub.
    pub emulator_host: Option<String>,
}
