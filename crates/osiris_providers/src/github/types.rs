//! GitHub's reads, in our terms.

use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

/// Where our GitHub App is installed: an account or organization.
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema, PartialEq, Eq)]
pub struct GithubInstallation {
    pub id: String,
    /// The account or organization's login.
    pub account: String,
    /// `user` or `organization` today. An open string: more values may come without a new
    /// release, and a client keeps working on one it doesn't know.
    #[schema(value_type = String)]
    pub account_type: GithubAccountType,
    /// `all` (every repository of the account) or `selected` (only the ones chosen at install)
    /// today. An open string: more values may come without a new release.
    #[schema(value_type = String)]
    pub repository_selection: GithubRepositorySelection,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, ToSchema, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum GithubAccountType {
    User,
    Organization,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, ToSchema, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum GithubRepositorySelection {
    /// Every repository of the account.
    All,
    /// Only the repositories chosen at install.
    Selected,
}

/// A repository our GitHub App reads, through one of its installations.
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema, PartialEq, Eq)]
pub struct GithubRepository {
    pub id: String,
    /// `owner/name`.
    pub full_name: String,
    pub private: bool,
    pub url: String,
    pub description: Option<String>,
}
