//! Someone who contributed to a repository, as GitHub answers it.

use crate::github::types::GithubContributor;
use serde::Deserialize;

#[derive(Deserialize)]
pub struct ContributorAnswer {
    pub id: u64,
    #[serde(default)]
    pub login: String,
    #[serde(default)]
    pub avatar_url: String,
    #[serde(rename = "html_url", default)]
    pub url: String,
    #[serde(rename = "type", default)]
    pub account_type: String,
    #[serde(default)]
    pub site_admin: bool,
    #[serde(default)]
    pub contributions: i64,
}

impl From<ContributorAnswer> for GithubContributor {
    /// Someone who contributed to a repository, in our terms.
    fn from(answer: ContributorAnswer) -> Self {
        Self {
            id: answer.id.to_string(),
            login: answer.login,
            avatar_url: answer.avatar_url,
            url: answer.url,
            account_type: answer.account_type,
            site_admin: answer.site_admin,
            contributions: answer.contributions,
        }
    }
}
