//! A transport that publishes events to Google Pub/Sub, using Google's official client
//! (`google-cloud-pubsub`). With an emulator host set, it talks to the emulator without
//! credentials. When deployed, it uses the service account's Application Default Credentials.

pub mod config;
pub mod listen;
pub mod transport;

pub use config::PubSubConfig;
pub use transport::PubSubTransport;
