//! `github.get_repository_readme`: a repository's README, its content in base64 and, when it is
//! text, decoded.

use crate::{
    ProviderError,
    github::{
        Github, path_segment::path_segment, request::send,
        transformers::content::FileContentAnswer, trimmed::trimmed, types::GithubFileContent,
    },
};
use reqwest::Method;
use serde::Deserialize;
use utoipa::ToSchema;

#[derive(Debug, Deserialize, ToSchema)]
#[serde(deny_unknown_fields)]
pub struct GetRepositoryReadme {
    pub owner: String,
    pub repo: String,
    /// A branch, a tag or a commit SHA; the default branch when left out.
    #[serde(rename = "ref")]
    pub reference: Option<String>,
}

impl GetRepositoryReadme {
    /// The scopes it needs, as `OpenConnector` names them.
    pub const SCOPES: &[&str] = &["repo"];
}

impl Github {
    /// `GET /repos/{owner}/{repo}/readme`. A README that is not a regular file is refused.
    pub async fn get_repository_readme(
        &self,
        http: &reqwest::Client,
        token: &str,
        input: &GetRepositoryReadme,
    ) -> Result<GithubFileContent, ProviderError> {
        let path = format!(
            "/repos/{}/{}/readme",
            path_segment(&input.owner),
            path_segment(&input.repo)
        );
        let query: Vec<(&str, String)> = trimmed(input.reference.as_deref())
            .map(|reference| ("ref", reference.to_owned()))
            .into_iter()
            .collect();
        let answer: FileContentAnswer =
            send::<_, ()>(self, http, token, Method::GET, &path, &query, None).await?;
        if answer.entry_type != "file" {
            return Err(ProviderError::refused(
                "github",
                "readme does not resolve to a regular file",
            ));
        }
        Ok(answer.into())
    }
}
