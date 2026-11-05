//! What the server tests share: the real API and worker routers on the test's own database,
//! providers pointed at a fake, an in-memory bus to read what was published, and an in-memory task
//! scheduler to read the scheduled tasks and run them through the worker.

#![allow(dead_code, reason = "each test file uses some of these")]
#![allow(
    clippy::unwrap_used,
    clippy::indexing_slicing,
    clippy::panic,
    reason = "test fixtures: a failure here is the test failing"
)]

use axum::{
    Router,
    body::Body,
    http::{Request, StatusCode},
};
use http_body_util::BodyExt;
use osiris_common::{crypto::MasterKey, secret::secret};
use osiris_events::{MessageBus, adapters::memory::MemoryTransport};
use osiris_providers::{ProviderUrls, Providers};
use osiris_server::{
    config::Settings,
    core::Core,
    http::state::ApiState,
    services::{api, worker},
};
use osiris_storage::Store;
use osiris_tasks::{
    Task,
    adapters::{bearer::BearerAuthenticator, memory::MemoryScheduler},
};
use serde_json::Value;
use sqlx::PgPool;
use std::sync::Arc;
use tower::ServiceExt;
use url::Url;
use wiremock::{
    Mock, MockServer, ResponseTemplate,
    matchers::{body_string_contains, method, path},
};

pub const API_KEY: &str = "test-api-key-0123456789abcdefghijklmn";
pub const ADMIN_KEY: &str = "test-admin-key-0123456789abcdefghijklm";

pub const WORKER_TOKEN: &str = "test-worker-token-0123456789abcdefg";

pub struct App {
    pub router: Router,
    pub worker: Router,
    pub core: Core,
    pub bus: Arc<MemoryTransport>,
    pub tasks: Arc<MemoryScheduler>,
    pub provider: MockServer,
    /// The worker's draining flag, as SIGTERM sets it.
    pub draining: Arc<std::sync::atomic::AtomicBool>,
}

pub async fn app(pool: PgPool) -> App {
    let provider = MockServer::start().await;
    let base = Url::parse(&provider.uri()).unwrap();
    let urls = ProviderUrls {
        oauth_base: base.clone(),
        api_base: base.clone(),
    };
    let slack = ProviderUrls {
        oauth_base: base.clone(),
        api_base: base.join("/api/").unwrap(),
    };
    let settings = Settings::default();
    let bus = Arc::new(MemoryTransport::default());
    let tasks = Arc::new(MemoryScheduler::default());
    let core = Core {
        store: Store::new(
            pool,
            MasterKey::from_secret("a-test-secret-that-is-long-enough-for-osiris").unwrap(),
        ),
        bus: MessageBus::new(bus.clone()),
        tasks: tasks.clone(),
        providers: Arc::new(
            Providers::new(&urls, &urls, &slack, settings.providers.listing.limits()).unwrap(),
        ),
        http: reqwest::Client::new(),
        public_url: "http://osiris.test".into(),
        settings: Arc::new(settings),
    };
    let state = ApiState {
        core: core.clone(),
        api_key: Arc::new(secret(API_KEY)),
        admin_key: Arc::new(secret(ADMIN_KEY)),
    };
    let draining = Arc::new(std::sync::atomic::AtomicBool::new(false));
    let worker = worker::router(worker::state::WorkerState {
        core: core.clone(),
        authenticator: Arc::new(BearerAuthenticator::new(WORKER_TOKEN.into()).unwrap()),
        draining: draining.clone(),
    });
    App {
        router: api::router(state),
        worker,
        core,
        bus,
        tasks,
        provider,
        draining,
    }
}

/// A request's answer: its status, headers and JSON body (null when none).
pub struct Answer {
    pub status: StatusCode,
    pub headers: axum::http::HeaderMap,
    pub body: Value,
}

impl App {
    pub async fn send(
        &self,
        method: &str,
        path: &str,
        key: Option<&str>,
        body: Option<Value>,
        extra: &[(&str, &str)],
    ) -> Answer {
        let mut request = Request::builder()
            .method(method)
            .uri(path)
            .header("content-type", "application/json");
        if let Some(key) = key {
            request = request.header("authorization", format!("Bearer {key}"));
        }
        for (name, value) in extra {
            request = request.header(*name, *value);
        }
        let body = body.map_or_else(Body::empty, |body| Body::from(body.to_string()));
        let response = self
            .router
            .clone()
            .oneshot(request.body(body).unwrap())
            .await
            .unwrap();
        let status = response.status();
        let headers = response.headers().clone();
        let bytes = response.into_body().collect().await.unwrap().to_bytes();
        Answer {
            status,
            headers,
            body: serde_json::from_slice(&bytes).unwrap_or(Value::Null),
        }
    }

    /// A GitHub auth config, through the admin API.
    pub async fn github_config(&self) -> String {
        let answer = self
            .send("POST", "/v1/auth-configs", Some(ADMIN_KEY), Some(serde_json::json!({ "provider": "github", "client_id": "Iv1.client", "client_secret": "secret" })), &[])
            .await;
        assert_eq!(answer.status, StatusCode::CREATED, "{}", answer.body);
        answer.body["id"].as_str().unwrap().to_owned()
    }

    /// Start a connect; answers the connection's id and the `state` of its consent URL.
    pub async fn start_connect(&self, auth_config_id: &str) -> (String, String) {
        let answer = self
            .send(
                "POST",
                "/v1/connected-accounts",
                Some(API_KEY),
                Some(serde_json::json!({ "auth_config_id": auth_config_id, "user_id": "org_1", "alias": "attempt-1", "callback_url": "http://app.test/integrations/callback?connection=row-1" })),
                &[],
            )
            .await;
        assert_eq!(answer.status, StatusCode::CREATED, "{}", answer.body);
        let redirect = Url::parse(answer.body["redirect_url"].as_str().unwrap()).unwrap();
        let state = redirect
            .query_pairs()
            .find(|(name, _)| name == "state")
            .unwrap()
            .1
            .into_owned();
        (answer.body["id"].as_str().unwrap().to_owned(), state)
    }

    /// Sends a raw body to the worker's `/tasks`, as the scheduler does, with `token`.
    pub async fn send_to_worker(&self, body: Vec<u8>, token: Option<&str>) -> StatusCode {
        let mut request = Request::builder()
            .method("POST")
            .uri("/tasks")
            .header("content-type", "application/json");
        if let Some(token) = token {
            request = request.header("authorization", format!("Bearer {token}"));
        }
        self.worker
            .clone()
            .oneshot(request.body(Body::from(body)).unwrap())
            .await
            .unwrap()
            .status()
    }

    /// Sends `task` to the worker, as the scheduler does when it is due.
    pub async fn deliver(&self, task: &Task) -> StatusCode {
        self.send_to_worker(serde_json::to_vec(task).unwrap(), Some(WORKER_TOKEN))
            .await
    }

    /// Takes the pending task `name` out of the scheduler and sends it to the worker.
    pub async fn run_task(&self, name: &str) -> StatusCode {
        let task = self.tasks.take(name).unwrap_or_else(|| {
            panic!(
                "no pending task {name}; pending: {:?}",
                self.tasks.pending()
            )
        });
        self.deliver(&task).await
    }

    /// The events published so far.
    pub fn events(&self) -> Vec<osiris_events::catalog::events::EventMessage> {
        self.bus
            .published()
            .into_iter()
            .map(|(_, envelope)| {
                let osiris_events::Message::Event(event) = envelope.message;
                event
            })
            .collect()
    }

    /// The outcomes recorded for the connection's task runs, oldest first.
    pub async fn outcomes(&self, connected_account_id: &str) -> Vec<String> {
        sqlx::query_scalar(
            "SELECT outcome FROM task_outcome WHERE connected_account_id = $1 ORDER BY id",
        )
        .bind(connected_account_id)
        .fetch_all(&self.core.store.pool)
        .await
        .unwrap()
    }

    /// The connection, as the API answers it.
    pub async fn account(&self, id: &str) -> Value {
        self.send(
            "GET",
            &format!("/v1/connected-accounts/{id}"),
            Some(API_KEY),
            None,
            &[],
        )
        .await
        .body
    }

    /// A GitHub connection, connected through the consent, whose token expires in
    /// `expires_in` seconds.
    pub async fn connected(&self, expires_in: i64) -> String {
        Mock::given(method("POST"))
            .and(path("/login/oauth/access_token"))
            .and(body_string_contains("grant_type=authorization_code"))
            .respond_with(ResponseTemplate::new(200).set_body_json(
                serde_json::json!({ "access_token": "ghu_1", "refresh_token": "ghr_1", "expires_in": expires_in }),
            ))
            .mount(&self.provider)
            .await;
        let config = self.github_config().await;
        let (id, state) = self.start_connect(&config).await;
        let back = self
            .send(
                "GET",
                &format!("/oauth/callback?state={state}&code=c"),
                None,
                None,
                &[],
            )
            .await;
        assert!(
            back.headers["location"]
                .to_str()
                .unwrap()
                .contains("status=active"),
            "{:?}",
            back.headers
        );
        id
    }

    /// A connection to `provider` through a new auth config, whose token endpoint is at
    /// `token_path` on the fake provider. The token expires in an hour and comes with a refresh
    /// token.
    pub async fn connected_to(&self, provider: &str, token_path: &str) -> String {
        Mock::given(method("POST"))
            .and(path(token_path))
            .respond_with(ResponseTemplate::new(200).set_body_json(
                serde_json::json!({ "ok": true, "access_token": format!("{provider}-token"), "refresh_token": format!("{provider}-refresh"), "expires_in": 3600 }),
            ))
            .mount(&self.provider)
            .await;
        let answer = self
            .send("POST", "/v1/auth-configs", Some(ADMIN_KEY), Some(serde_json::json!({ "provider": provider, "client_id": "client", "client_secret": "secret" })), &[])
            .await;
        assert_eq!(answer.status, StatusCode::CREATED, "{}", answer.body);
        let (id, state) = self
            .start_connect(answer.body["id"].as_str().unwrap())
            .await;
        let back = self
            .send(
                "GET",
                &format!("/oauth/callback?state={state}&code=c"),
                None,
                None,
                &[],
            )
            .await;
        assert!(
            back.headers["location"]
                .to_str()
                .unwrap()
                .contains("status=active"),
            "{:?}",
            back.headers
        );
        id
    }

    /// Sends a GET to the worker, as Cloud Run's health checks do.
    pub async fn send_to_worker_get(&self, path: &str) -> StatusCode {
        self.worker
            .clone()
            .oneshot(Request::builder().uri(path).body(Body::empty()).unwrap())
            .await
            .unwrap()
            .status()
    }
}
