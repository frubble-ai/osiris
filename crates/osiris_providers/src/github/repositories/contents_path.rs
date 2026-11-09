//! The path of a repository's contents API, for a file or a directory.

use crate::github::path_segment::path_segment;

/// `/repos/{owner}/{repo}/contents/{path}`, as `OpenConnector` builds it: the path's leading and
/// trailing slashes dropped, each of its parts percent-encoded on its own, and the repository's
/// root when it is left out or empty.
pub fn contents_path(owner: &str, repo: &str, path: Option<&str>) -> String {
    let parts: Vec<String> = path
        .unwrap_or_default()
        .split('/')
        .filter(|part| !part.is_empty())
        .map(path_segment)
        .collect();
    let root = format!(
        "/repos/{}/{}/contents",
        path_segment(owner),
        path_segment(repo)
    );
    if parts.is_empty() {
        root
    } else {
        format!("{root}/{}", parts.join("/"))
    }
}
