//! A branch, as GitHub answers it.

use crate::github::types::{GithubBranch, GithubBranchProtection};
use serde::Deserialize;

#[derive(Deserialize)]
pub struct BranchAnswer {
    name: String,
    commit: BranchCommitAnswer,
    #[serde(default)]
    protected: bool,
    protection: Option<BranchProtectionAnswer>,
}

#[derive(Deserialize)]
pub struct BranchProtectionAnswer {
    #[serde(default)]
    enabled: bool,
    required_status_checks: Option<RequiredChecksAnswer>,
}

#[derive(Deserialize)]
pub struct RequiredChecksAnswer {
    enforcement_level: Option<String>,
    #[serde(default)]
    contexts: Vec<String>,
}

impl From<BranchProtectionAnswer> for GithubBranchProtection {
    /// A branch's protection in our terms.
    fn from(answer: BranchProtectionAnswer) -> Self {
        let checks = answer.required_status_checks;
        Self {
            enabled: answer.enabled,
            enforcement_level: checks
                .as_ref()
                .and_then(|checks| checks.enforcement_level.clone()),
            required_status_checks: checks.map(|checks| checks.contexts).unwrap_or_default(),
        }
    }
}

#[derive(Deserialize)]
struct BranchCommitAnswer {
    sha: String,
}

impl From<BranchAnswer> for GithubBranch {
    /// The branch in our terms.
    fn from(answer: BranchAnswer) -> Self {
        Self {
            name: answer.name,
            commit_sha: answer.commit.sha,
            protected: answer.protected,
            protection: answer.protection.map(GithubBranchProtection::from),
        }
    }
}
