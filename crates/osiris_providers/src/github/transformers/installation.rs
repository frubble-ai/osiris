//! An installation, as `GET /user/installations` answers it.

use crate::github::types::GithubInstallation;
use serde::Deserialize;
use std::collections::BTreeMap;

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
    #[serde(default)]
    app_slug: String,
    #[serde(default)]
    permissions: BTreeMap<String, String>,
    #[serde(default)]
    events: Vec<String>,
    created_at: Option<String>,
    updated_at: Option<String>,
    suspended_at: Option<String>,
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
            app_slug: answer.app_slug,
            permissions: answer.permissions,
            events: answer.events,
            created_at: answer.created_at,
            updated_at: answer.updated_at,
            suspended_at: answer.suspended_at,
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
