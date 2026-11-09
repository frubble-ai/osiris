//! A topic a search found, as GitHub answers it.

use super::{
    commit::CommitAnswer, issue::IssueAnswer, repository::RepositoryAnswer, user::UserAnswer,
};
use crate::github::types::{
    GithubCommit, GithubIssue, GithubRepository, GithubSearchCode, GithubSearchCodeRepository,
    GithubSearchCommit, GithubSearchCommitRepository, GithubSearchIssue, GithubSearchLabel,
    GithubSearchRepository, GithubSearchTopic, GithubSearchUser, GithubUser,
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
    #[serde(flatten)]
    pub issue: IssueAnswer,
    #[serde(default)]
    pub repository_url: String,
    #[serde(default)]
    pub score: f64,
}

impl From<SearchIssueAnswer> for GithubSearchIssue {
    /// An issue or a pull request a search found, in our terms.
    fn from(answer: SearchIssueAnswer) -> Self {
        Self {
            issue: GithubIssue::from(answer.issue),
            repository_url: answer.repository_url,
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
    #[serde(flatten)]
    pub commit: CommitAnswer,
    pub repository: SearchCommitRepositoryAnswer,
    #[serde(default)]
    pub score: f64,
}

impl From<SearchCommitAnswer> for GithubSearchCommit {
    /// A commit a search found, in our terms.
    fn from(answer: SearchCommitAnswer) -> Self {
        Self {
            commit: GithubCommit::from(answer.commit),
            repository: GithubSearchCommitRepository::from(answer.repository),
            score: answer.score,
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
    #[serde(default)]
    pub score: f64,
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
            score: answer.score,
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
    pub default: bool,
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
            default: answer.default,
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
    pub created_by: Option<String>,
    pub released: Option<String>,
    pub created_at: Option<String>,
    pub updated_at: Option<String>,
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
            created_by: answer.created_by,
            released: answer.released,
            created_at: answer.created_at,
            updated_at: answer.updated_at,
            score: answer.score,
        }
    }
}
