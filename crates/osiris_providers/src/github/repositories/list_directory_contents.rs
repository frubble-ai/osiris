//! `github.list_directory_contents`: the entries of a directory of a repository, the root when no
//! path is given.

use crate::{
    ProviderError,
    github::{
        Github,
        repositories::{contents_path::contents_path, trimmed::trimmed},
        request::send,
        transformers::content::ContentsAnswer,
        types::GithubContentEntry,
    },
};
use reqwest::Method;
use serde::Deserialize;
use utoipa::ToSchema;

#[derive(Debug, Deserialize, ToSchema)]
#[serde(deny_unknown_fields)]
pub struct ListDirectoryContents {
    pub owner: String,
    pub repo: String,
    /// The directory, such as `src/lib`; the repository's root when left out or empty.
    pub path: Option<String>,
    /// A branch, a tag or a commit SHA; the default branch when left out.
    #[serde(rename = "ref")]
    pub reference: Option<String>,
}

impl ListDirectoryContents {
    /// The scopes it needs, as `OpenConnector` names them.
    pub const SCOPES: &[&str] = &["repo"];
}

impl Github {
    /// `GET /repos/{owner}/{repo}/contents/{path}`. A path that names a file, not a directory, is
    /// refused.
    pub async fn list_directory_contents(
        &self,
        http: &reqwest::Client,
        token: &str,
        input: &ListDirectoryContents,
    ) -> Result<Vec<GithubContentEntry>, ProviderError> {
        let path = contents_path(&input.owner, &input.repo, trimmed(input.path.as_deref()));
        let query: Vec<(&str, String)> = trimmed(input.reference.as_deref())
            .map(|reference| ("ref", reference.to_owned()))
            .into_iter()
            .collect();
        let answer: ContentsAnswer =
            send::<_, ()>(self, http, token, Method::GET, &path, &query, None).await?;
        match answer {
            ContentsAnswer::Directory(entries) => {
                Ok(entries.into_iter().map(GithubContentEntry::from).collect())
            }
            ContentsAnswer::Item(_) => Err(ProviderError::refused(
                "github",
                "path does not resolve to a directory",
            )),
        }
    }
}
