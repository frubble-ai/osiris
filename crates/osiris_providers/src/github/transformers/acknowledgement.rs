//! An action that answers nothing of its own: `ok` once GitHub has done it, as GitHub answers it.

use crate::github::types::GithubAcknowledgement;
use serde::Deserialize;

#[derive(Deserialize)]
pub struct AcknowledgementAnswer {
    #[serde(default)]
    pub ok: bool,
}

impl From<AcknowledgementAnswer> for GithubAcknowledgement {
    /// An action that answers nothing of its own: `ok` once GitHub has done it, in our terms.
    fn from(answer: AcknowledgementAnswer) -> Self {
        Self { ok: answer.ok }
    }
}
