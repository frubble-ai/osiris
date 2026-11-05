//! One page of a GitHub REST list.

use super::Github;
use crate::ProviderError;
use osiris_interfaces::call::{Request, call};
use reqwest::Method;

/// GitHub's largest page, a fact of its API.
pub const PER_PAGE: usize = 100;

pub async fn get_page<T: serde::de::DeserializeOwned>(
    github: &Github,
    http: &reqwest::Client,
    token: &str,
    path: &str,
    page: u32,
) -> Result<T, ProviderError> {
    call::<T, ()>(
        github,
        http,
        token,
        Request {
            method: Method::GET,
            path,
            query: &[
                ("per_page", PER_PAGE.to_string()),
                ("page", page.to_string()),
            ],
            headers: &[
                ("accept", "application/vnd.github+json"),
                ("x-github-api-version", "2022-11-28"),
            ],
            body: None,
        },
    )
    .await
}
