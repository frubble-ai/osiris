//! A label, as GitHub answers it.

use crate::github::types::GithubLabel;
use serde::Deserialize;

#[derive(Deserialize)]
pub struct LabelAnswer {
    pub id: u64,
    #[serde(default)]
    pub name: String,
    #[serde(default)]
    pub color: String,
    pub description: Option<String>,
    #[serde(default)]
    pub default: bool,
}

impl From<LabelAnswer> for GithubLabel {
    /// A label, in our terms.
    fn from(answer: LabelAnswer) -> Self {
        Self {
            id: answer.id.to_string(),
            name: answer.name,
            color: answer.color,
            description: answer.description,
            default: answer.default,
        }
    }
}
