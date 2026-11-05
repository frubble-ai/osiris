//! The one way a provider's API is called: the provider's base URL and auth header, and its errors
//! read by the provider itself.

use crate::{ProviderCommon, ProviderError};
use reqwest::Method;
use serde::{Serialize, de::DeserializeOwned};

/// A request to `path` (resolved against the provider's base URL) with the user's `token`.
pub struct Request<'a, B: Serialize> {
    pub method: Method,
    pub path: &'a str,
    pub query: &'a [(&'a str, String)],
    pub headers: &'a [(&'a str, &'a str)],
    pub body: Option<&'a B>,
}

pub async fn call<T: DeserializeOwned, B: Serialize>(
    provider: &dyn ProviderCommon,
    http: &reqwest::Client,
    token: &str,
    request: Request<'_, B>,
) -> Result<T, ProviderError> {
    let url = provider.base_url().join(request.path).map_err(|error| {
        ProviderError::unavailable(provider.id(), format!("{}: {error}", request.path))
    })?;
    let (header, value) = provider.auth_header(token);
    let mut builder = http
        .request(request.method, url)
        .header(header, value)
        .query(request.query);
    for (name, value) in request.headers {
        builder = builder.header(*name, *value);
    }
    if let Some(body) = request.body {
        builder = builder.json(body);
    }
    let response = builder
        .send()
        .await
        .map_err(|error| ProviderError::unavailable(provider.id(), error))?;
    let status = response.status();
    let body = response
        .text()
        .await
        .map_err(|error| ProviderError::unavailable(provider.id(), error))?;
    if !status.is_success() {
        return Err(provider.parse_error(status, &body));
    }
    serde_json::from_str(&body).map_err(|error| ProviderError::unavailable(provider.id(), error))
}
