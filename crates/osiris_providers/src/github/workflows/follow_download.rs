//! A GitHub endpoint that answers with a redirect to a file it keeps elsewhere (a job's logs, an
//! artifact's archive), followed to the file.
//!
//! The shared [`send`](crate::github::request::send) reads every answer as JSON, and these files
//! are plain text or a ZIP archive, so this request stops at the response and leaves its body to
//! the caller. It goes through the client osiris passes in, which follows redirects (reqwest's
//! default, at most ten), so it keeps the configured timeouts and user agent. A second client that
//! does not follow redirects would need timeouts of its own, which are settings this provider does
//! not have. reqwest drops the `Authorization` header when a redirect leaves GitHub's host, so the
//! token never reaches the storage the file is kept in.

use crate::{
    ProviderCommon, ProviderError,
    github::{Github, request::API_VERSION},
};

/// `GET path` as the user `token`, with the headers GitHub asks every request to carry, followed
/// through GitHub's redirect. The answer is the response at the end of the redirects, its body
/// not yet read; an error answer, from GitHub or from where it redirects, reads as GitHub's
/// errors do. No error names the URL, since the one GitHub redirects to is signed.
pub async fn follow_download(
    github: &Github,
    http: &reqwest::Client,
    token: &str,
    path: &str,
) -> Result<reqwest::Response, ProviderError> {
    let url = github
        .base_url()
        .join(path)
        .map_err(|error| ProviderError::unavailable(github.id(), format!("{path}: {error}")))?;
    let (header, value) = github.auth_header(token);
    let response = http
        .get(url)
        .header(header, value)
        .header("accept", "application/vnd.github+json")
        .header("x-github-api-version", API_VERSION)
        .send()
        .await
        .map_err(|error| ProviderError::unavailable(github.id(), error.without_url()))?;
    let status = response.status();
    if status.is_success() {
        return Ok(response);
    }
    let body = response
        .text()
        .await
        .map_err(|error| ProviderError::unavailable(github.id(), error.without_url()))?;
    Err(github.parse_error(status, &body))
}
