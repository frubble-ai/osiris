//! `github.get_label`: a repository's label, by name.

use crate::{
    ProviderError,
    github::{
        Github, path_segment::path_segment, request::send, transformers::label::LabelAnswer,
        types::GithubLabel,
    },
};
use reqwest::Method;
use serde::Deserialize;
use utoipa::ToSchema;

#[derive(Debug, Deserialize, ToSchema)]
#[serde(deny_unknown_fields)]
pub struct GetLabel {
    pub owner: String,
    pub repo: String,
    pub name: String,
}

impl GetLabel {
    /// The scopes it needs, as `OpenConnector` names them.
    pub const SCOPES: &[&str] = &["repo"];
}

impl Github {
    /// `GET /repos/{owner}/{repo}/labels/{name}`.
    pub async fn get_label(
        &self,
        http: &reqwest::Client,
        token: &str,
        input: &GetLabel,
    ) -> Result<GithubLabel, ProviderError> {
        let path = format!(
            "/repos/{}/{}/labels/{}",
            path_segment(&input.owner),
            path_segment(&input.repo),
            path_segment(&input.name)
        );
        let answer: LabelAnswer =
            send::<_, ()>(self, http, token, Method::GET, &path, &[], None).await?;
        Ok(answer.into())
    }
}
