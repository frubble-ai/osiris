//! GitHub's answers as it sends them, and into our types.

use super::types::{
    GithubAccountType, GithubInstallation, GithubRepository, GithubRepositorySelection,
};
use serde::Deserialize;

/// A page of `GET /user/installations`.
#[derive(Deserialize)]
pub struct InstallationsPage {
    pub total_count: u64,
    pub installations: Vec<InstallationAnswer>,
}

#[derive(Deserialize)]
pub struct InstallationAnswer {
    id: u64,
    account: AccountAnswer,
    repository_selection: SelectionAnswer,
}

#[derive(Deserialize)]
struct AccountAnswer {
    login: String,
    #[serde(rename = "type")]
    kind: String,
}

#[derive(Deserialize)]
#[serde(rename_all = "snake_case")]
enum SelectionAnswer {
    All,
    Selected,
}

/// A page of `GET /user/installations/{id}/repositories`.
#[derive(Deserialize)]
pub struct RepositoriesPage {
    pub total_count: u64,
    pub repositories: Vec<RepositoryAnswer>,
}

#[derive(Deserialize)]
pub struct RepositoryAnswer {
    pub id: u64,
    full_name: String,
    private: bool,
    html_url: String,
    description: Option<String>,
}

impl From<InstallationAnswer> for GithubInstallation {
    fn from(answer: InstallationAnswer) -> Self {
        Self {
            id: answer.id.to_string(),
            account: answer.account.login,
            // GitHub also answers "Enterprise"; it holds organizations, so it reads as one.
            account_type: if answer.account.kind == "User" {
                GithubAccountType::User
            } else {
                GithubAccountType::Organization
            },
            repository_selection: match answer.repository_selection {
                SelectionAnswer::All => GithubRepositorySelection::All,
                SelectionAnswer::Selected => GithubRepositorySelection::Selected,
            },
        }
    }
}

impl From<RepositoryAnswer> for GithubRepository {
    fn from(answer: RepositoryAnswer) -> Self {
        Self {
            id: answer.id.to_string(),
            full_name: answer.full_name,
            private: answer.private,
            url: answer.html_url,
            description: answer.description,
        }
    }
}
