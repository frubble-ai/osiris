//! One request to GitHub's REST API, with the headers GitHub asks every request to carry.

use super::Github;
use crate::ProviderError;
use osiris_interfaces::call::{Request, call};
use reqwest::Method;
use serde::{Serialize, de::DeserializeOwned};

/// GitHub's REST API version every request names, a fact of its API.
pub const API_VERSION: &str = "2022-11-28";

/// `method path` with `query` and `body`, as the user `token`; an empty answer (a 204) reads as
/// `()` or `None`.
pub async fn send<T: DeserializeOwned, B: Serialize>(
    github: &Github,
    http: &reqwest::Client,
    token: &str,
    method: Method,
    path: &str,
    query: &[(&str, String)],
    body: Option<&B>,
) -> Result<T, ProviderError> {
    call::<T, B>(
        github,
        http,
        token,
        Request {
            method,
            path,
            query,
            headers: &[
                ("accept", "application/vnd.github+json"),
                ("x-github-api-version", API_VERSION),
            ],
            body,
        },
    )
    .await
}
