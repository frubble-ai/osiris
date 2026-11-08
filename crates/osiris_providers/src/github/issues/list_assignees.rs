//! `github.list_assignees`: the people issues in a repository can be assigned to, a page at a
//! time.

use crate::{
    ProviderError,
    github::{
        Github, paging::paging, path_segment::path_segment, request::send,
        transformers::user::UserAnswer, types::GithubUser,
    },
};
use reqwest::Method;
use serde::Deserialize;
use utoipa::ToSchema;

#[derive(Debug, Deserialize, ToSchema)]
#[serde(deny_unknown_fields)]
pub struct ListAssignees {
    pub owner: String,
    pub repo: String,
    /// At most 100.
    pub per_page: Option<u32>,
    /// From 1.
    pub page: Option<u32>,
}

impl ListAssignees {
    /// The scopes it needs, as `OpenConnector` names them.
    pub const SCOPES: &[&str] = &["repo"];
}

impl Github {
    /// `GET /repos/{owner}/{repo}/assignees`.
    pub async fn list_assignees(
        &self,
        http: &reqwest::Client,
        token: &str,
        input: &ListAssignees,
    ) -> Result<Vec<GithubUser>, ProviderError> {
        let path = format!(
            "/repos/{}/{}/assignees",
            path_segment(&input.owner),
            path_segment(&input.repo)
        );
        let query = paging(input.per_page, input.page);
        let answer: Vec<UserAnswer> =
            send::<_, ()>(self, http, token, Method::GET, &path, &query, None).await?;
        Ok(answer.into_iter().map(GithubUser::from).collect())
    }
}
