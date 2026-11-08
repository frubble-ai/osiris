//! Where a workflow artifact's ZIP archive can be downloaded.

use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

/// Where a workflow artifact's ZIP archive can be downloaded.
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema, PartialEq)]
pub struct GithubArtifactDownload {
    /// A signed URL that downloads the archive without a token, for one minute.
    pub url: String,
}
