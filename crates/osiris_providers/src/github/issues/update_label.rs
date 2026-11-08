//! `github.update_label`: a label renamed, recolored or described anew.

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

/// Only the fields given are changed.
#[derive(Debug, Deserialize, ToSchema)]
#[serde(deny_unknown_fields)]
pub struct UpdateLabel {
    pub owner: String,
    pub repo: String,
    /// The label's name now.
    pub name: String,
    /// The label's name from now on.
    pub new_name: Option<String>,
    /// The label's color, as six hex digits without `#`; anything else is refused before GitHub
    /// is asked.
    pub color: Option<String>,
    pub description: Option<String>,
}

impl UpdateLabel {
    /// The scopes it needs, as `OpenConnector` names them.
    pub const SCOPES: &[&str] = &["repo"];
}

#[derive(Serialize)]
struct UpdateLabelBody<'a> {
    #[serde(skip_serializing_if = "Option::is_none")]
    new_name: Option<&'a str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    color: Option<&'a str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    description: Option<&'a str>,
}

impl Github {
    /// `PATCH /repos/{owner}/{repo}/labels/{name}`, refused without a call when the color is not
    /// six hex digits.
    pub async fn update_label(
        &self,
        http: &reqwest::Client,
        token: &str,
        input: &UpdateLabel,
    ) -> Result<GithubLabel, ProviderError> {
        if let Some(color) = &input.color
            && !(color.len() == 6 && color.bytes().all(|byte| byte.is_ascii_hexdigit()))
        {
            return Err(ProviderError::refused(
                "github",
                "color must be a 6-character hex color without #",
            ));
        }
        let path = format!(
            "/repos/{}/{}/labels/{}",
            path_segment(&input.owner),
            path_segment(&input.repo),
            path_segment(&input.name)
        );
        let body = UpdateLabelBody {
            new_name: input.new_name.as_deref(),
            color: input.color.as_deref(),
            description: input.description.as_deref(),
        };
        let answer: LabelAnswer =
            send(self, http, token, Method::PATCH, &path, &[], Some(&body)).await?;
        Ok(answer.into())
    }
}
