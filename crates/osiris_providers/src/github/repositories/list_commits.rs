//! `github.list_commits`: a repository's commits, newest first, filtered by branch, path, author
//! and date, a page at a time.

use crate::{
    ProviderError,
    github::{
        Github, paging::paging, path_segment::path_segment, request::send,
        transformers::commit::CommitAnswer, trimmed::trimmed, types::GithubCommit,
    },
};
use reqwest::Method;
use serde::Deserialize;
use utoipa::ToSchema;

#[derive(Debug, Deserialize, ToSchema)]
#[serde(deny_unknown_fields)]
pub struct ListCommits {
    pub owner: String,
    pub repo: String,
    /// A branch name or a commit SHA to list from; the default branch when left out.
    pub sha: Option<String>,
    /// Only commits that changed this file or directory.
    pub path: Option<String>,
    /// Only commits by this login or email.
    pub author: Option<String>,
    /// Only commits committed by this login or email.
    pub committer: Option<String>,
    /// Only commits from this time on, in ISO 8601.
    pub since: Option<String>,
    /// Only commits before this time, in ISO 8601.
    pub until: Option<String>,
    /// At most 100.
    pub per_page: Option<u32>,
    /// From 1.
    pub page: Option<u32>,
}

impl ListCommits {
    /// The scopes it needs, as `OpenConnector` names them.
    pub const SCOPES: &[&str] = &["repo"];
}

impl Github {
    /// `GET /repos/{owner}/{repo}/commits`.
    pub async fn list_commits(
        &self,
        http: &reqwest::Client,
        token: &str,
        input: &ListCommits,
    ) -> Result<Vec<GithubCommit>, ProviderError> {
        let path = format!(
            "/repos/{}/{}/commits",
            path_segment(&input.owner),
            path_segment(&input.repo)
        );
        let mut query = Vec::new();
        for (name, value) in [
            ("sha", &input.sha),
            ("path", &input.path),
            ("author", &input.author),
            ("committer", &input.committer),
            ("since", &input.since),
            ("until", &input.until),
        ] {
            if let Some(value) = trimmed(value.as_deref()) {
                query.push((name, value.to_owned()));
            }
        }
        query.extend(paging(input.per_page, input.page));
        let answer: Vec<CommitAnswer> =
            send::<_, ()>(self, http, token, Method::GET, &path, &query, None).await?;
        Ok(answer.into_iter().map(GithubCommit::from).collect())
    }
}
