//! The test context, built for each test from its own database: the real use cases over in-memory
//! adapters, and the API and worker routers over those use cases.

use super::test_config::build_test_config;
use crate::{
    config::Config,
    http::state::ApiState,
    services::{api, worker},
    use_cases::{UseCaseContext, UseCases},
};
use axum::Router;
use osiris_common::{
    crypto::MasterKey,
    secret::{ExposeSecret, secret},
};
use osiris_events::{MessageBus, adapters::memory::MemoryTransport};
use osiris_providers::{ProviderUrls, Providers};
use osiris_storage::Store;
use osiris_tasks::adapters::{bearer::BearerAuthenticator, memory::MemoryScheduler};
use sqlx::PgPool;
use std::sync::{Arc, atomic::AtomicBool};
use url::Url;
use wiremock::MockServer;

pub const API_KEY: &str = "test-api-key-0123456789abcdefghijklmn";
pub const ADMIN_KEY: &str = "test-admin-key-0123456789abcdefghijklm";
pub const WORKER_TOKEN: &str = "test-worker-token-0123456789abcdefg";

pub struct TestContext {
    pub use_cases: UseCases,
    pub config: Arc<Config>,
    pub api: Router,
    pub worker: Router,
    /// What was published.
    pub bus: Arc<MemoryTransport>,
    /// What was scheduled.
    pub tasks: Arc<MemoryScheduler>,
    /// Every provider's network.
    pub provider: MockServer,
    /// The worker's draining flag, as SIGTERM sets it.
    pub draining: Arc<AtomicBool>,
}

/// A test context with the configuration fixture.
pub async fn setup_test_context(pool: PgPool) -> TestContext {
    setup_test_context_with(pool, |_| {}).await
}

/// A test context whose configuration fixture `configure` changes first.
pub async fn setup_test_context_with(
    pool: PgPool,
    configure: impl FnOnce(&mut Config),
) -> TestContext {
    let provider = MockServer::start().await;
    let base = Url::parse(&provider.uri()).expect("the fake's URL");
    let urls = ProviderUrls {
        oauth_base: base.clone(),
        api_base: base.clone(),
    };
    let slack = ProviderUrls {
        oauth_base: base.clone(),
        api_base: base.join("/api/").expect("Slack's API path"),
    };
    let mut config = build_test_config();
    configure(&mut config);
    let config = Arc::new(config);
    let settings = &config.settings;
    let bus = Arc::new(MemoryTransport::default());
    let tasks = Arc::new(MemoryScheduler::default());
    let use_cases = UseCases::new(UseCaseContext {
        store: Store::new(
            pool,
            MasterKey::from_secret(config.encryption_key.expose_secret()).expect("the test key"),
            settings.database.lock_timeout(),
        ),
        messages: MessageBus::new(bus.clone()),
        tasks: tasks.clone(),
        providers: Arc::new(
            Providers::new(&urls, &urls, &slack, settings.providers.listing.limits())
                .expect("the fake's URLs"),
        ),
        http: reqwest::Client::new(),
        config: config.clone(),
    });
    let draining = Arc::new(AtomicBool::new(false));
    let api = api::router(ApiState {
        use_cases: use_cases.clone(),
        config: config.clone(),
        api_key: Arc::new(secret(API_KEY)),
        admin_key: Arc::new(secret(ADMIN_KEY)),
    });
    let worker = worker::router(worker::state::WorkerState {
        use_cases: use_cases.clone(),
        config: config.clone(),
        authenticator: Arc::new(
            BearerAuthenticator::new(secret(WORKER_TOKEN)).expect("a long enough token"),
        ),
        draining: draining.clone(),
    });
    TestContext {
        use_cases,
        config,
        api,
        worker,
        bus,
        tasks,
        provider,
        draining,
    }
}
