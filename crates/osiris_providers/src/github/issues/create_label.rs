//! `github.create_label`: a new label in a repository.

use crate::{
    ProviderError,
    github::{
        Github, path_segment::path_segment, request::send, transformers::label::LabelAnswer,
        types::GithubLabel,
    },
};
use reqwest::Method;
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

#[derive(Debug, Deserialize, ToSchema)]
#[serde(deny_unknown_fields)]
pub struct CreateLabel {
    pub owner: String,
    pub repo: String,
    pub name: String,
    /// The label's color, as six hex digits without `#`.
    pub color: String,
    pub description: Option<String>,
}

impl CreateLabel {
    /// The scopes it needs, as `OpenConnector` names them.
    pub const SCOPES: &[&str] = &["repo"];
}

#[derive(Serialize)]
struct CreateLabelBody<'a> {
    name: &'a str,
    color: &'a str,
    #[serde(skip_serializing_if = "Option::is_none")]
    description: Option<&'a str>,
}

impl Github {
    /// `POST /repos/{owner}/{repo}/labels`.
    pub async fn create_label(
        &self,
        http: &reqwest::Client,
        token: &str,
        input: &CreateLabel,
    ) -> Result<GithubLabel, ProviderError> {
        let path = format!(
            "/repos/{}/{}/labels",
            path_segment(&input.owner),
            path_segment(&input.repo)
        );
        let body = CreateLabelBody {
            name: &input.name,
            color: &input.color,
            description: input.description.as_deref(),
        };
        let answer: LabelAnswer =
            send(self, http, token, Method::POST, &path, &[], Some(&body)).await?;
        Ok(answer.into())
    }
}
