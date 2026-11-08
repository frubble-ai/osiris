//! `github.update_release`: a release changed, by its id.

use crate::{
    ProviderError,
    github::{
        Github, path_segment::path_segment, request::send, transformers::release::ReleaseAnswer,
        trimmed::trimmed, types::GithubRelease,
    },
};
use reqwest::Method;
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

/// Only the fields given change.
#[derive(Debug, Deserialize, ToSchema)]
#[serde(deny_unknown_fields)]
pub struct UpdateRelease {
    /// The account or organization that owns the repository.
    pub owner: String,
    pub repo: String,
    /// GitHub's numeric id of the release.
    pub release_id: u64,
    pub tag_name: Option<String>,
    /// The branch or commit SHA a new tag is created from.
    pub target_commitish: Option<String>,
    pub name: Option<String>,
    /// The release's description, in Markdown.
    pub body: Option<String>,
    /// A draft is not published; `false` publishes it.
    pub draft: Option<bool>,
    /// Marks the release as not ready for production.
    pub prerelease: Option<bool>,
    /// Whether it becomes the repository's latest release: `true`, `false` or `legacy` (the
    /// latest by creation date and version).
    pub make_latest: Option<String>,
}

impl UpdateRelease {
    /// The scopes it needs, as `OpenConnector` names them.
    pub const SCOPES: &[&str] = &["repo"];
}

/// What `PATCH /repos/{owner}/{repo}/releases/{release_id}` takes.
#[derive(Serialize)]
struct UpdateReleaseBody<'a> {
    #[serde(skip_serializing_if = "Option::is_none")]
    tag_name: Option<&'a str>,
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
    make_latest: Option<&'a str>,
}

impl Github {
    /// `PATCH /repos/{owner}/{repo}/releases/{release_id}`.
    pub async fn update_release(
        &self,
        http: &reqwest::Client,
        token: &str,
        input: &UpdateRelease,
    ) -> Result<GithubRelease, ProviderError> {
        let path = format!(
            "/repos/{}/{}/releases/{}",
            path_segment(&input.owner),
            path_segment(&input.repo),
            path_segment(&input.release_id.to_string())
        );
        let body = UpdateReleaseBody {
            tag_name: trimmed(input.tag_name.as_deref()),
            target_commitish: trimmed(input.target_commitish.as_deref()),
            name: input.name.as_deref(),
            body: input.body.as_deref(),
            draft: input.draft,
            prerelease: input.prerelease,
            make_latest: trimmed(input.make_latest.as_deref()),
        };
        let answer: ReleaseAnswer =
            send(self, http, token, Method::PATCH, &path, &[], Some(&body)).await?;
        Ok(answer.into())
    }
}
