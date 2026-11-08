//! GitHub's answers as it sends them, and into our types.

use super::types::{GithubInstallation, GithubRepository};
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
    /// `all` or `selected` today, passed on as GitHub sends it.
    repository_selection: String,
}

#[derive(Deserialize)]
struct AccountAnswer {
    login: String,
    #[serde(rename = "type")]
    kind: String,
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
    /// The installation in our terms, its account type and selection passed on as open strings.
    fn from(answer: InstallationAnswer) -> Self {
        Self {
            id: answer.id.to_string(),
            account: answer.account.login,
            account_type: account_type(&answer.account.kind),
            repository_selection: answer.repository_selection,
        }
    }
}

impl From<RepositoryAnswer> for GithubRepository {
    /// The repository in our terms.
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

/// GitHub's account `type` in our words: `User` and `Organization` as `user` and `organization`.
/// `Enterprise` holds organizations, so it reads as one; any other type GitHub adds is passed on in
/// lower case, since the field is an open string.
fn account_type(kind: &str) -> String {
    match kind {
        "User" => "user".to_owned(),
        "Organization" | "Enterprise" => "organization".to_owned(),
        other => other.to_ascii_lowercase(),
    }
}
