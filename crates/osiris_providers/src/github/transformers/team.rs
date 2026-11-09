//! A team of an organization, as GitHub answers it.

use crate::github::types::GithubTeam;
use serde::Deserialize;

#[derive(Deserialize)]
pub struct TeamAnswer {
    pub id: u64,
    #[serde(default)]
    pub name: String,
    #[serde(default)]
    pub slug: String,
}

impl From<TeamAnswer> for GithubTeam {
    /// A team of an organization, in our terms.
    fn from(answer: TeamAnswer) -> Self {
        Self {
            id: answer.id.to_string(),
            name: answer.name,
            slug: answer.slug,
        }
    }
}
