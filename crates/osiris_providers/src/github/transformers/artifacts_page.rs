//! A page of a workflow run's artifacts, as GitHub wraps it.

use super::artifact::ArtifactAnswer;
use serde::Deserialize;

/// A page of a workflow run's artifacts, as GitHub wraps it.
#[derive(Deserialize)]
pub struct ArtifactsPage {
    #[serde(default)]
    pub artifacts: Vec<ArtifactAnswer>,
}
