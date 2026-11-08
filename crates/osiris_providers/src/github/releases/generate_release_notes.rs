//! `github.generate_release_notes`: the name and body GitHub writes for a release of a tag, from
//! the changes since the previous release. Nothing is saved.

use crate::{
    ProviderError,
    github::{
        Github, path_segment::path_segment, request::send,
        transformers::release::ReleaseNotesAnswer, types::GithubReleaseNotes,
    },
};
use reqwest::Method;
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

#[derive(Debug, Deserialize, ToSchema)]
#[serde(deny_unknown_fields)]
pub struct GenerateReleaseNotes {
    /// The account or organization that owns the repository.
    pub owner: String,
    pub repo: String,
    /// The tag the notes are for. It need not exist yet.
    pub tag_name: String,
    /// The branch or commit SHA the tag would be created from, when it doesn't exist yet.
    pub target_commitish: Option<String>,
    /// The tag the notes start from. GitHub picks the previous release's when left out.
    pub previous_tag_name: Option<String>,
    /// The path in the repository of a configuration file for the notes. GitHub reads
    /// `.github/release.yml` when left out.
    pub configuration_file_path: Option<String>,
}

impl GenerateReleaseNotes {
    /// The scopes it needs, as `OpenConnector` names them.
    pub const SCOPES: &[&str] = &["repo"];
}

/// What `POST /repos/{owner}/{repo}/releases/generate-notes` takes.
#[derive(Serialize)]
struct GenerateReleaseNotesBody<'a> {
    tag_name: &'a str,
    #[serde(skip_serializing_if = "Option::is_none")]
    target_commitish: Option<&'a str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    previous_tag_name: Option<&'a str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    configuration_file_path: Option<&'a str>,
}

impl Github {
    /// `POST /repos/{owner}/{repo}/releases/generate-notes`.
    pub async fn generate_release_notes(
        &self,
        http: &reqwest::Client,
        token: &str,
        input: &GenerateReleaseNotes,
    ) -> Result<GithubReleaseNotes, ProviderError> {
        let path = format!(
            "/repos/{}/{}/releases/generate-notes",
            path_segment(&input.owner),
            path_segment(&input.repo)
        );
        let body = GenerateReleaseNotesBody {
            tag_name: &input.tag_name,
            target_commitish: input.target_commitish.as_deref(),
            previous_tag_name: input.previous_tag_name.as_deref(),
            configuration_file_path: input.configuration_file_path.as_deref(),
        };
        let answer: ReleaseNotesAnswer =
            send(self, http, token, Method::POST, &path, &[], Some(&body)).await?;
        Ok(answer.into())
    }
}
