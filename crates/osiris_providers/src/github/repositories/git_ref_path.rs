//! A git reference placed in a path, such as `heads/main`, each of its parts percent-encoded on
//! its own so that its slashes still separate them.

use crate::{ProviderError, github::path_segment::path_segment};

/// `reference` with each part between its slashes percent-encoded.
pub fn git_ref_path(reference: &str) -> String {
    reference
        .split('/')
        .map(path_segment)
        .collect::<Vec<_>>()
        .join("/")
}

/// As [`git_ref_path`], for a branch or a tag only: a reference that doesn't start with `heads/`
/// or `tags/` is refused before GitHub is asked, as `OpenConnector` refuses it.
pub fn branch_or_tag_ref_path(reference: &str) -> Result<String, ProviderError> {
    if reference.starts_with("heads/") || reference.starts_with("tags/") {
        Ok(git_ref_path(reference))
    } else {
        Err(ProviderError::invalid(
            "github",
            "ref must start with heads/ or tags/",
        ))
    }
}
