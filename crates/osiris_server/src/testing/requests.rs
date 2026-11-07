//! Requests to the routers, as clients, Cloud Run's probes and the task scheduler send them.

use super::{TestContext, WORKER_TOKEN, conformance::assert_answer_conforms};
use axum::{
    body::Body,
    http::{HeaderMap, Request, StatusCode},
};
use http_body_util::BodyExt;
use osiris_tasks::Task;
use serde_json::Value;
use tower::ServiceExt;

/// A request's answer: its status, headers and JSON body (`null` when there is none), already
/// checked against its release's spec.
pub struct Answer {
    pub status: StatusCode,
    pub headers: HeaderMap,
    pub body: Value,
}

impl TestContext {
    /// Sends a request to the API, with `key` as its bearer key and `extra` headers.
    pub async fn send(
        &self,
        method: &str,
        path: &str,
        key: Option<&str>,
        body: Option<Value>,
        extra: &[(&str, &str)],
    ) -> Answer {
        let body = body.map(|body| body.to_string()).unwrap_or_default();
        self.send_raw(method, path, key, body, extra).await
    }

    /// Sends `body` as it is, JSON or not, as a request with a JSON content type.
    pub async fn send_raw(
        &self,
        method: &str,
        path: &str,
        key: Option<&str>,
        body: String,
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
        let body = if body.is_empty() {
            Body::empty()
        } else {
            Body::from(body)
        };
        let response = self
            .api
            .clone()
            .oneshot(request.body(body).expect("a request"))
            .await
            .expect("an answer");
        let status = response.status();
        let headers = response.headers().clone();
        let bytes = response
            .into_body()
            .collect()
            .await
            .expect("a body")
            .to_bytes();
        let body = serde_json::from_slice(&bytes).unwrap_or(Value::Null);
        assert_answer_conforms(self.releases, method, path, status, &headers, &body);
        Answer {
            status,
            headers,
            body,
        }
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
            .oneshot(request.body(Body::from(body)).expect("a request"))
            .await
            .expect("an answer")
            .status()
    }

    /// Sends a GET to the worker, as Cloud Run's probes do.
    pub async fn send_to_worker_get(&self, path: &str) -> StatusCode {
        self.worker
            .clone()
            .oneshot(
                Request::builder()
                    .uri(path)
                    .body(Body::empty())
                    .expect("a request"),
            )
            .await
            .expect("an answer")
            .status()
    }

    /// Sends `task` to the worker, as the scheduler does when it is due.
    pub async fn deliver(&self, task: &Task) -> StatusCode {
        self.send_to_worker(
            serde_json::to_vec(task).expect("a task's JSON"),
            Some(WORKER_TOKEN),
        )
        .await
    }

    /// Takes the pending task `name` out of the scheduler and sends it to the worker.
    pub async fn deliver_pending(&self, name: &str) -> StatusCode {
        let task = self.take_pending(name);
        self.deliver(&task).await
    }

    /// Takes the pending task `name` out of the scheduler, as it does when the task is due.
    pub fn take_pending(&self, name: &str) -> Task {
        self.tasks.take(name).unwrap_or_else(|| {
            panic!(
                "no pending task {name}; pending: {:?}",
                self.tasks.pending()
            )
        })
    }
}
