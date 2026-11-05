//! What the core works with, built once per process.

use crate::config::{Settings, settings::http::HttpSettings};
use osiris_events::MessageBus;
use osiris_providers::Providers;
use osiris_storage::Store;
use osiris_tasks::TaskScheduler;
use std::sync::Arc;

#[derive(Clone)]
pub struct Core {
    pub store: Store,
    pub bus: MessageBus,
    /// The scheduler that runs osiris's scheduled tasks (token refreshes, consent expiries and
    /// event retries).
    pub tasks: Arc<dyn TaskScheduler>,
    pub providers: Arc<Providers>,
    pub http: reqwest::Client,
    /// Osiris's public origin, for the OAuth redirect URI.
    pub public_url: String,
    pub settings: Arc<Settings>,
}

impl Core {
    /// Where providers send the browser back after a consent.
    pub fn redirect_uri(&self) -> String {
        format!("{}/oauth/callback", self.public_url)
    }
}

/// The HTTP client for providers: bounded, so a slow provider can't hold a request.
pub fn http_client(settings: &HttpSettings) -> reqwest::Result<reqwest::Client> {
    reqwest::Client::builder()
        .user_agent("osiris")
        .connect_timeout(settings.connect_timeout())
        .timeout(settings.timeout())
        .build()
}
