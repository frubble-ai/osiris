//! Where our GitHub App is installed.

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
    pub account_type: String,
    /// `all` (every repository of the account) or `selected` (only the ones chosen at install)
    /// today. An open string: more values may come without a new release.
    pub repository_selection: String,
}
