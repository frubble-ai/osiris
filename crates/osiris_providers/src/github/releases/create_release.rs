//! `github.create_release`: a new release of a repository, from a tag.

use crate::{
    ProviderError,
    github::{
        Github, path_segment::path_segment, request::send, transformers::release::ReleaseAnswer,
        types::GithubRelease,
    },
};
use reqwest::Method;
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

#[derive(Debug, Deserialize, ToSchema)]
#[serde(deny_unknown_fields)]
pub struct CreateRelease {
    /// The account or organization that owns the repository.
    pub owner: String,
    pub repo: String,
    /// The tag the release is made from. GitHub creates it when it doesn't exist yet.
    pub tag_name: String,
    /// The branch or commit SHA a new tag is created from. The repository's default branch when
    /// left out.
    pub target_commitish: Option<String>,
    pub name: Option<String>,
    /// The release's description, in Markdown.
    pub body: Option<String>,
    /// A draft is not published.
    pub draft: Option<bool>,
    /// Marks the release as not ready for production.
    pub prerelease: Option<bool>,
    /// GitHub writes the release's name and body from the changes since the previous release,
    /// after any `name` and `body` given.
    pub generate_release_notes: Option<bool>,
    /// Whether it becomes the repository's latest release: `true`, `false` or `legacy` (the
    /// latest by creation date and version).
    pub make_latest: Option<String>,
}

impl CreateRelease {
    /// The scopes it needs, as `OpenConnector` names them.
    pub const SCOPES: &[&str] = &["repo"];
}

/// What `POST /repos/{owner}/{repo}/releases` takes.
#[derive(Serialize)]
struct CreateReleaseBody<'a> {
    tag_name: &'a str,
    #[serde(skip_serializing_if = "Option::is_none")]
    target_commitish: Option<&'a str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    name: Option<&'a str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    body: Option<&'a str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    draft: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    prerelease: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    generate_release_notes: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    make_latest: Option<&'a str>,
}

impl Github {
    /// `POST /repos/{owner}/{repo}/releases`.
    pub async fn create_release(
        &self,
        http: &reqwest::Client,
        token: &str,
        input: &CreateRelease,
    ) -> Result<GithubRelease, ProviderError> {
        let path = format!(
            "/repos/{}/{}/releases",
            path_segment(&input.owner),
            path_segment(&input.repo)
        );
        let body = CreateReleaseBody {
            tag_name: &input.tag_name,
            target_commitish: input.target_commitish.as_deref(),
            name: input.name.as_deref(),
            body: input.body.as_deref(),
            draft: input.draft,
            prerelease: input.prerelease,
            generate_release_notes: input.generate_release_notes,
            make_latest: input.make_latest.as_deref(),
        };
        let answer: ReleaseAnswer =
            send(self, http, token, Method::POST, &path, &[], Some(&body)).await?;
        Ok(answer.into())
    }
}
