//! A request's path, checked before it is resolved against a provider's base URL. A segment of `.`
//! or `..`, as given or percent-encoded (`%2e`), would be collapsed by the URL parser and move
//! the request to another endpoint, such as from a file in a repository to the repository
//! itself. A path holding one is refused, so no input can reach an endpoint its action doesn't
//! name.

use crate::ProviderError;

/// `path`, or [`ProviderError::Invalid`] when a segment of it is `.` or `..`.
pub fn request_path<'a>(provider: &'static str, path: &'a str) -> Result<&'a str, ProviderError> {
    let route = path.split(['?', '#']).next().unwrap_or(path);
    if route.split('/').any(is_dot_segment) {
        return Err(ProviderError::invalid(
            provider,
            "a path segment can't be `.` or `..`",
        ));
    }
    Ok(path)
}

/// Whether `segment` is `.` or `..`, with its dots percent-encoded or not.
fn is_dot_segment(segment: &str) -> bool {
    let decoded = segment.to_ascii_lowercase().replace("%2e", ".");
    decoded == "." || decoded == ".."
}

#[cfg(test)]
mod tests;
