//! How many of each reaction an issue or a comment has, as GitHub answers it.

use crate::github::types::GithubReactions;
use serde::Deserialize;

#[derive(Deserialize, Default)]
pub struct ReactionsAnswer {
    #[serde(default)]
    pub total_count: i64,
    #[serde(rename = "+1", default)]
    pub plus_one: i64,
    #[serde(rename = "-1", default)]
    pub minus_one: i64,
    #[serde(default)]
    pub laugh: i64,
    #[serde(default)]
    pub confused: i64,
    #[serde(default)]
    pub heart: i64,
    #[serde(default)]
    pub hooray: i64,
    #[serde(default)]
    pub rocket: i64,
    #[serde(default)]
    pub eyes: i64,
}

impl From<ReactionsAnswer> for GithubReactions {
    /// How many of each reaction, in our terms.
    fn from(answer: ReactionsAnswer) -> Self {
        Self {
            total_count: answer.total_count,
            plus_one: answer.plus_one,
            minus_one: answer.minus_one,
            laugh: answer.laugh,
            confused: answer.confused,
            heart: answer.heart,
            hooray: answer.hooray,
            rocket: answer.rocket,
            eyes: answer.eyes,
        }
    }
}
