//! A branch.

use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

/// A branch: its name, the commit it points at, and whether it is protected.
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema, PartialEq, Eq)]
pub struct GithubBranch {
    pub name: String,
    pub commit_sha: String,
    pub protected: bool,
    /// What its protection asks of a change, when GitHub answers it.
    pub protection: Option<GithubBranchProtection>,
}

/// What a branch's protection asks of a change before it lands.
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema, PartialEq, Eq)]
pub struct GithubBranchProtection {
    pub enabled: bool,
    /// Who the required status checks apply to: `off`, `non_admins` or `everyone`.
    pub enforcement_level: Option<String>,
    /// The status checks that must pass, by context.
    pub required_status_checks: Vec<String>,
}
