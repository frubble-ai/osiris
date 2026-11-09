//! A topic a search found, as GitHub answers it.

use super::{repository::RepositoryAnswer, user::UserAnswer};
use crate::github::types::{
    GithubRepository, GithubSearchCode, GithubSearchCodeRepository, GithubSearchCommit,
    GithubSearchCommitRepository, GithubSearchIssue, GithubSearchLabel, GithubSearchRepository,
    GithubSearchTopic, GithubSearchUser, GithubUser,
};
use serde::Deserialize;

#[derive(Deserialize)]
pub struct SearchRepositoryAnswer {
    #[serde(flatten)]
    pub repository: RepositoryAnswer,
    #[serde(default)]
    pub score: f64,
}

impl From<SearchRepositoryAnswer> for GithubSearchRepository {
    /// A repository a search found, in our terms.
    fn from(answer: SearchRepositoryAnswer) -> Self {
        Self {
            repository: GithubRepository::from(answer.repository),
            score: answer.score,
        }
    }
}

#[derive(Deserialize)]
pub struct SearchIssueAnswer {
    pub id: u64,
    #[serde(default)]
    pub number: i64,
    #[serde(default)]
    pub title: String,
    #[serde(rename = "html_url", default)]
    pub url: String,
    #[serde(default)]
    pub state: String,
    pub body: Option<String>,
    #[serde(default)]
    pub repository_url: String,
    #[serde(rename = "pull_request")]
    pub is_pull_request: Option<serde_json::Value>,
    pub user: Option<UserAnswer>,
    #[serde(default)]
    pub score: f64,
}

impl From<SearchIssueAnswer> for GithubSearchIssue {
    /// An issue or a pull request a search found, in our terms.
    fn from(answer: SearchIssueAnswer) -> Self {
        Self {
            id: answer.id.to_string(),
            number: answer.number,
            title: answer.title,
            url: answer.url,
            state: answer.state,
            body: answer.body,
            repository_url: answer.repository_url,
            is_pull_request: answer.is_pull_request.is_some(),
            user: answer.user.map(GithubUser::from),
            score: answer.score,
        }
    }
}

#[derive(Deserialize)]
pub struct SearchUserAnswer {
    pub id: u64,
    #[serde(default)]
    pub login: String,
    #[serde(rename = "type", default)]
    pub account_type: String,
    #[serde(rename = "html_url", default)]
    pub url: String,
    #[serde(default)]
    pub avatar_url: String,
    #[serde(default)]
    pub score: f64,
}

impl From<SearchUserAnswer> for GithubSearchUser {
    /// A user a search found, in our terms.
    fn from(answer: SearchUserAnswer) -> Self {
        Self {
            id: answer.id.to_string(),
            login: answer.login,
            account_type: answer.account_type,
            url: answer.url,
            avatar_url: answer.avatar_url,
            score: answer.score,
        }
    }
}

#[derive(Deserialize)]
pub struct SearchCommitRepositoryAnswer {
    pub id: u64,
    #[serde(default)]
    pub full_name: String,
    #[serde(rename = "html_url", default)]
    pub url: String,
}

impl From<SearchCommitRepositoryAnswer> for GithubSearchCommitRepository {
    /// The repository a found commit is in, in our terms.
    fn from(answer: SearchCommitRepositoryAnswer) -> Self {
        Self {
            id: answer.id.to_string(),
            full_name: answer.full_name,
            url: answer.url,
        }
    }
}

#[derive(Deserialize)]
pub struct SearchCommitAnswer {
    #[serde(default)]
    pub sha: String,
    #[serde(rename = "html_url", default)]
    pub url: String,
    pub author: Option<UserAnswer>,
    pub committer: Option<UserAnswer>,
    pub repository: SearchCommitRepositoryAnswer,
    #[serde(default)]
    pub score: f64,
    #[serde(default)]
    pub commit: Option<SearchCommitCommitAnswer>,
}

#[derive(Deserialize, Default)]
pub struct SearchCommitCommitAnswer {
    #[serde(default)]
    pub message: String,
}

impl From<SearchCommitAnswer> for GithubSearchCommit {
    /// A commit a search found, in our terms.
    fn from(answer: SearchCommitAnswer) -> Self {
        let commit = answer.commit.unwrap_or_default();
        Self {
            sha: answer.sha,
            url: answer.url,
            author: answer.author.map(GithubUser::from),
            committer: answer.committer.map(GithubUser::from),
            repository: GithubSearchCommitRepository::from(answer.repository),
            score: answer.score,
            message: commit.message,
        }
    }
}

#[derive(Deserialize)]
pub struct SearchCodeRepositoryAnswer {
    pub id: u64,
    #[serde(default)]
    pub full_name: String,
    #[serde(rename = "html_url", default)]
    pub url: String,
    pub owner: Option<UserAnswer>,
}

impl From<SearchCodeRepositoryAnswer> for GithubSearchCodeRepository {
    /// The repository found code is in, in our terms.
    fn from(answer: SearchCodeRepositoryAnswer) -> Self {
        Self {
            id: answer.id.to_string(),
            full_name: answer.full_name,
            url: answer.url,
            owner: answer.owner.map(GithubUser::from),
        }
    }
}

#[derive(Deserialize)]
pub struct SearchCodeAnswer {
    #[serde(default)]
    pub name: String,
    #[serde(default)]
    pub path: String,
    #[serde(default)]
    pub sha: String,
    #[serde(rename = "html_url", default)]
    pub url: String,
    #[serde(default)]
    pub git_url: String,
    pub repository: SearchCodeRepositoryAnswer,
}

impl From<SearchCodeAnswer> for GithubSearchCode {
    /// A file a code search found, in our terms.
    fn from(answer: SearchCodeAnswer) -> Self {
        Self {
            name: answer.name,
            path: answer.path,
            sha: answer.sha,
            url: answer.url,
            git_url: answer.git_url,
            repository: GithubSearchCodeRepository::from(answer.repository),
        }
    }
}

#[derive(Deserialize)]
pub struct SearchLabelAnswer {
    pub id: u64,
    #[serde(default)]
    pub name: String,
    #[serde(default)]
    pub color: String,
    pub description: Option<String>,
    #[serde(default)]
    pub score: f64,
}

impl From<SearchLabelAnswer> for GithubSearchLabel {
    /// A label a search found, in our terms.
    fn from(answer: SearchLabelAnswer) -> Self {
        Self {
            id: answer.id.to_string(),
            name: answer.name,
            color: answer.color,
            description: answer.description,
            score: answer.score,
        }
    }
}

#[derive(Deserialize)]
pub struct SearchTopicAnswer {
    #[serde(default)]
    pub name: String,
    pub display_name: Option<String>,
    pub short_description: Option<String>,
    pub description: Option<String>,
    #[serde(default)]
    pub featured: bool,
    #[serde(default)]
    pub curated: bool,
    #[serde(default)]
    pub score: f64,
}

impl From<SearchTopicAnswer> for GithubSearchTopic {
    /// A topic a search found, in our terms.
    fn from(answer: SearchTopicAnswer) -> Self {
        Self {
            name: answer.name,
            display_name: answer.display_name,
            short_description: answer.short_description,
            description: answer.description,
            featured: answer.featured,
            curated: answer.curated,
            score: answer.score,
        }
    }
}
