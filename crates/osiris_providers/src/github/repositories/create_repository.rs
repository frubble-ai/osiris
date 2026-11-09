//! `github.create_repository`: a new repository of the authenticated user's.

use crate::{
    ProviderError,
    github::{
        Github, request::send, transformers::repository::RepositoryAnswer, trimmed::trimmed,
        types::GithubRepository,
    },
};
use reqwest::Method;
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

#[derive(Debug, Deserialize, ToSchema)]
#[serde(deny_unknown_fields)]
#[allow(
    clippy::struct_excessive_bools,
    reason = "GitHub's own settings, one each"
)]
pub struct CreateRepository {
    pub name: String,
    pub description: Option<String>,
    /// A URL with more about the repository.
    pub homepage: Option<String>,
    pub private: Option<bool>,
    /// Whether GitHub makes a first commit with an empty README.
    pub auto_init: Option<bool>,
    pub has_issues: Option<bool>,
    pub has_projects: Option<bool>,
    pub has_wiki: Option<bool>,
    pub has_discussions: Option<bool>,
    /// The name of a `.gitignore` template GitHub offers, such as `Rust`.
    pub gitignore_template: Option<String>,
    /// The keyword of a license GitHub offers, such as `mit`.
    pub license_template: Option<String>,
}

impl CreateRepository {
    /// The scopes it needs, as `OpenConnector` names them.
    pub const SCOPES: &[&str] = &["repo"];
}

#[derive(Serialize)]
struct Body<'a> {
    name: &'a str,
    #[serde(skip_serializing_if = "Option::is_none")]
    description: Option<&'a str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    homepage: Option<&'a str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    private: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    auto_init: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    has_issues: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    has_projects: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    has_wiki: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    has_discussions: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    gitignore_template: Option<&'a str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    license_template: Option<&'a str>,
}

impl Github {
    /// `POST /user/repos`.
    pub async fn create_repository(
        &self,
        http: &reqwest::Client,
        token: &str,
        input: &CreateRepository,
    ) -> Result<GithubRepository, ProviderError> {
        let body = Body {
            name: &input.name,
            description: input.description.as_deref(),
            homepage: trimmed(input.homepage.as_deref()),
            private: input.private,
            auto_init: input.auto_init,
            has_issues: input.has_issues,
            has_projects: input.has_projects,
            has_wiki: input.has_wiki,
            has_discussions: input.has_discussions,
            gitignore_template: trimmed(input.gitignore_template.as_deref()),
            license_template: trimmed(input.license_template.as_deref()),
        };
        let answer: RepositoryAnswer = send(
            self,
            http,
            token,
            Method::POST,
            "/user/repos",
            &[],
            Some(&body),
        )
        .await?;
        Ok(answer.into())
    }
}
