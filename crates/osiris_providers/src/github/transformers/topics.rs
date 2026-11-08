//! A repository's topics, as GitHub answers them.

use crate::github::types::GithubTopics;
use serde::Deserialize;

#[derive(Deserialize)]
pub struct TopicsAnswer {
    #[serde(default)]
    pub names: Vec<String>,
}

impl From<TopicsAnswer> for GithubTopics {
    /// A repository's topics, in our terms.
    fn from(answer: TopicsAnswer) -> Self {
        Self {
            names: answer.names,
        }
    }
}
