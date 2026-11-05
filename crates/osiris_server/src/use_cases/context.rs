//! What the use cases run on: the configuration and the configured clients, built once at boot.
//! Nothing here runs a flow of its own, and nothing about a request is kept here; requests reach a
//! use case through its arguments.

use super::lib::tasks::Scheduling;
use crate::config::Config;
use crate::use_cases::lib::database::Db;
use osiris_events::MessageBus;
use osiris_providers::Providers;
use osiris_tasks::TaskScheduler;
use std::sync::Arc;

pub struct UseCaseContext {
    pub config: Arc<Config>,
    pub db: Db,
    /// Where events for clients are published.
    pub messages: MessageBus,
    /// What runs scheduled tasks (token refreshes, consent expiries and event retries).
    pub tasks: Arc<dyn TaskScheduler>,
    pub providers: Arc<Providers>,
    /// The HTTP client for providers.
    pub http: reqwest::Client,
}

impl UseCaseContext {
    /// The task scheduler with the configured timeout, as the helpers take it.
    pub fn scheduling(&self) -> Scheduling<'_> {
        Scheduling {
            tasks: self.tasks.as_ref(),
            timeout: self.config.settings.tasks.schedule_timeout(),
        }
    }
}
