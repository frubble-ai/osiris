//! `fake.whoami`: who the token belongs to, the one action the shared tests run.

use super::FakeProvider;
use crate::ProviderError;
use osiris_interfaces::call::{Request, call};
use reqwest::Method;
use serde_json::Value;

impl FakeProvider {
    /// `GET /fake/me`.
    pub async fn whoami(
        &self,
        http: &reqwest::Client,
        token: &str,
    ) -> Result<Value, ProviderError> {
        call::<Value, ()>(
            self,
            http,
            token,
            Request {
                method: Method::GET,
                path: "/fake/me",
                query: &[],
                headers: &[],
                body: None,
            },
        )
        .await
    }
}
