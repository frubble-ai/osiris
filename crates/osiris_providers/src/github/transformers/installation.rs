//! An installation, as `GET /user/installations` answers it.

use crate::github::types::GithubInstallation;
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
