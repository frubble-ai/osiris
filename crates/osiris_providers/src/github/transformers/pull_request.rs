//! A pull request, as GitHub answers it.

use super::{label::LabelAnswer, milestone::MilestoneAnswer, user::UserAnswer};
use crate::github::types::{
    GithubLabel, GithubMilestone, GithubPullRequest, GithubPullRequestBranch, GithubUser,
};
use serde::Deserialize;

#[derive(Deserialize)]
pub struct PullRequestBranchAnswer {
    #[serde(rename = "ref", default)]
    pub ref_name: String,
    #[serde(default)]
    pub sha: String,
    #[serde(default)]
    pub label: String,
    #[serde(default)]
    pub repo: Option<PullRequestBranchRepoAnswer>,
}

#[derive(Deserialize, Default)]
pub struct PullRequestBranchRepoAnswer {
    #[serde(rename = "full_name", default)]
    pub repository_full_name: Option<String>,
}

impl From<PullRequestBranchAnswer> for GithubPullRequestBranch {
    /// The branch a pull request comes from (`head`) or goes into (`base`), in our terms.
    fn from(answer: PullRequestBranchAnswer) -> Self {
        let repo = answer.repo.unwrap_or_default();
        Self {
            ref_name: answer.ref_name,
            sha: answer.sha,
            label: answer.label,
            repository_full_name: repo.repository_full_name,
        }
    }
}

#[derive(Deserialize)]
#[allow(
    clippy::struct_excessive_bools,
    reason = "GitHub's own flags, one each"
)]
pub struct PullRequestAnswer {
    pub id: u64,
    #[serde(default)]
    pub number: i64,
    #[serde(default)]
    pub state: String,
    #[serde(default)]
    pub title: String,
    pub body: Option<String>,
    #[serde(rename = "html_url", default)]
    pub url: String,
    #[serde(default)]
    pub draft: bool,
    #[serde(default)]
    pub merged: bool,
    pub user: Option<UserAnswer>,
    pub head: PullRequestBranchAnswer,
    pub base: PullRequestBranchAnswer,
    #[serde(default)]
    pub created_at: String,
    #[serde(default)]
    pub updated_at: String,
    pub closed_at: Option<String>,
    pub merged_at: Option<String>,
    #[serde(default)]
    pub labels: Vec<LabelAnswer>,
    #[serde(default)]
    pub assignees: Vec<UserAnswer>,
    pub milestone: Option<MilestoneAnswer>,
    #[serde(default)]
    pub requested_reviewers: Vec<UserAnswer>,
    #[serde(default)]
    pub requested_teams: Vec<TeamAnswer>,
    pub mergeable: Option<bool>,
    pub mergeable_state: Option<String>,
    pub merge_commit_sha: Option<String>,
    pub maintainer_can_modify: Option<bool>,
    pub auto_merge: Option<serde_json::Value>,
    pub commits: Option<i64>,
    pub additions: Option<i64>,
    pub deletions: Option<i64>,
    pub changed_files: Option<i64>,
    pub comments: Option<i64>,
    pub review_comments: Option<i64>,
    #[serde(default)]
    pub locked: bool,
    pub active_lock_reason: Option<String>,
    pub merged_by: Option<UserAnswer>,
    pub rebaseable: Option<bool>,
    #[serde(default)]
    pub author_association: String,
}

/// A team asked to review, named by its slug.
#[derive(Deserialize)]
pub struct TeamAnswer {
    #[serde(default)]
    pub slug: String,
}

impl From<PullRequestAnswer> for GithubPullRequest {
    /// A pull request, in our terms.
    fn from(answer: PullRequestAnswer) -> Self {
        Self {
            id: answer.id.to_string(),
            number: answer.number,
            state: answer.state,
            title: answer.title,
            body: answer.body,
            url: answer.url,
            draft: answer.draft,
            // A list's pull requests carry no `merged`, only `merged_at`.
            merged: answer.merged || answer.merged_at.is_some(),
            user: answer.user.map(GithubUser::from),
            head: GithubPullRequestBranch::from(answer.head),
            base: GithubPullRequestBranch::from(answer.base),
            created_at: answer.created_at,
            updated_at: answer.updated_at,
            closed_at: answer.closed_at,
            merged_at: answer.merged_at,
            labels: answer.labels.into_iter().map(GithubLabel::from).collect(),
            assignees: answer.assignees.into_iter().map(GithubUser::from).collect(),
            milestone: answer.milestone.map(GithubMilestone::from),
            requested_reviewers: answer
                .requested_reviewers
                .into_iter()
                .map(GithubUser::from)
                .collect(),
            requested_teams: answer
                .requested_teams
                .into_iter()
                .map(|team| team.slug)
                .collect(),
            mergeable: answer.mergeable,
            mergeable_state: answer.mergeable_state,
            merge_commit_sha: answer.merge_commit_sha,
            maintainer_can_modify: answer.maintainer_can_modify,
            auto_merge: answer.auto_merge.is_some_and(|merge| !merge.is_null()),
            commits: answer.commits,
            additions: answer.additions,
            deletions: answer.deletions,
            changed_files: answer.changed_files,
            comments: answer.comments,
            review_comments: answer.review_comments,
            locked: answer.locked,
            active_lock_reason: answer.active_lock_reason,
            merged_by: answer.merged_by.map(GithubUser::from),
            rebaseable: answer.rebaseable,
            author_association: answer.author_association,
        }
    }
}
