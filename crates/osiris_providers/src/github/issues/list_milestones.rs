//! `github.list_milestones`: a repository's milestones, a page at a time.

use crate::{
    ProviderError,
    github::{
        Github, paging::paging, path_segment::path_segment, request::send,
        transformers::milestone::MilestoneAnswer, types::GithubMilestone,
    },
};
use reqwest::Method;
use serde::Deserialize;
use utoipa::ToSchema;

#[derive(Debug, Deserialize, ToSchema)]
#[serde(deny_unknown_fields)]
pub struct ListMilestones {
    pub owner: String,
    pub repo: String,
    /// `open` (GitHub's default), `closed` or `all`.
    pub state: Option<String>,
    /// `due_on` (GitHub's default) or `completeness`.
    pub sort: Option<String>,
    /// `asc` (GitHub's default) or `desc`.
    pub direction: Option<String>,
    /// At most 100.
    pub per_page: Option<u32>,
    /// From 1.
    pub page: Option<u32>,
}

impl ListMilestones {
    /// The scopes it needs, as `OpenConnector` names them.
    pub const SCOPES: &[&str] = &["repo"];
}

impl Github {
    /// `GET /repos/{owner}/{repo}/milestones`.
    pub async fn list_milestones(
        &self,
        http: &reqwest::Client,
        token: &str,
        input: &ListMilestones,
    ) -> Result<Vec<GithubMilestone>, ProviderError> {
        let path = format!(
            "/repos/{}/{}/milestones",
            path_segment(&input.owner),
            path_segment(&input.repo)
        );
        let mut query = paging(input.per_page, input.page);
        if let Some(state) = &input.state {
            query.push(("state", state.clone()));
        }
        if let Some(sort) = &input.sort {
            query.push(("sort", sort.clone()));
        }
        if let Some(direction) = &input.direction {
            query.push(("direction", direction.clone()));
        }
        let answer: Vec<MilestoneAnswer> =
            send::<_, ()>(self, http, token, Method::GET, &path, &query, None).await?;
        Ok(answer.into_iter().map(GithubMilestone::from).collect())
    }
}
