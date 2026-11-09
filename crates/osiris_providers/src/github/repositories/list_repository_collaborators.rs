//! `github.list_repository_collaborators`: a repository's collaborators, a page at a time.

use crate::{
    ProviderError,
    github::{
        Github, paging::paging, path_segment::path_segment, request::send,
        transformers::collaborator::CollaboratorAnswer, trimmed::trimmed,
        types::GithubCollaborator,
    },
};
use reqwest::Method;
use serde::Deserialize;
use utoipa::ToSchema;

#[derive(Debug, Deserialize, ToSchema)]
#[serde(deny_unknown_fields)]
pub struct ListRepositoryCollaborators {
    pub owner: String,
    pub repo: String,
    /// `outside`, `direct` or `all`.
    pub affiliation: Option<String>,
    /// Only those with this permission: `pull`, `triage`, `push`, `maintain` or `admin`.
    pub permission: Option<String>,
    /// At most 100.
    pub per_page: Option<u32>,
    /// From 1.
    pub page: Option<u32>,
}

impl ListRepositoryCollaborators {
    /// The scopes it needs, as `OpenConnector` names them.
    pub const SCOPES: &[&str] = &["repo"];
}

impl Github {
    /// `GET /repos/{owner}/{repo}/collaborators`.
    pub async fn list_repository_collaborators(
        &self,
        http: &reqwest::Client,
        token: &str,
        input: &ListRepositoryCollaborators,
    ) -> Result<Vec<GithubCollaborator>, ProviderError> {
        let path = format!(
            "/repos/{}/{}/collaborators",
            path_segment(&input.owner),
            path_segment(&input.repo)
        );
        let mut query = Vec::new();
        for (name, value) in [
            ("affiliation", &input.affiliation),
            ("permission", &input.permission),
        ] {
            if let Some(value) = trimmed(value.as_deref()) {
                query.push((name, value.to_owned()));
            }
        }
        query.extend(paging(input.per_page, input.page));
        let answer: Vec<CollaboratorAnswer> =
            send::<_, ()>(self, http, token, Method::GET, &path, &query, None).await?;
        Ok(answer.into_iter().map(GithubCollaborator::from).collect())
    }
}
