//! `github.list_repository_languages`: the languages a repository is written in, by bytes of code,
//! the most used first.

use crate::{
    ProviderError,
    github::{
        Github, path_segment::path_segment, request::send, transformers::language::LanguagesAnswer,
        types::GithubLanguage,
    },
};
use reqwest::Method;
use serde::Deserialize;
use utoipa::ToSchema;

#[derive(Debug, Deserialize, ToSchema)]
#[serde(deny_unknown_fields)]
pub struct ListRepositoryLanguages {
    pub owner: String,
    pub repo: String,
}

impl ListRepositoryLanguages {
    /// The scopes it needs, as `OpenConnector` names them.
    pub const SCOPES: &[&str] = &["repo"];
}

impl Github {
    /// `GET /repos/{owner}/{repo}/languages`, which GitHub answers as one map of each language's
    /// name to its bytes.
    pub async fn list_repository_languages(
        &self,
        http: &reqwest::Client,
        token: &str,
        input: &ListRepositoryLanguages,
    ) -> Result<Vec<GithubLanguage>, ProviderError> {
        let path = format!(
            "/repos/{}/{}/languages",
            path_segment(&input.owner),
            path_segment(&input.repo)
        );
        let answer: LanguagesAnswer =
            send::<_, ()>(self, http, token, Method::GET, &path, &[], None).await?;
        Ok(answer.into_languages())
    }
}
