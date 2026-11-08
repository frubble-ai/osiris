//! `github.get_file_contents`: a file of a repository, its content in base64 and, when it is
//! text, decoded.

use crate::{
    ProviderError,
    github::{
        Github,
        repositories::{contents_path::contents_path, trimmed::trimmed},
        request::send,
        transformers::content::ContentsAnswer,
        types::GithubFileContent,
    },
};
use reqwest::Method;
use serde::Deserialize;
use utoipa::ToSchema;

#[derive(Debug, Deserialize, ToSchema)]
#[serde(deny_unknown_fields)]
pub struct GetFileContents {
    pub owner: String,
    pub repo: String,
    /// The file, such as `src/lib.rs`.
    pub path: String,
    /// A branch, a tag or a commit SHA; the default branch when left out.
    #[serde(rename = "ref")]
    pub reference: Option<String>,
}

impl GetFileContents {
    /// The scopes it needs, as `OpenConnector` names them.
    pub const SCOPES: &[&str] = &["repo"];
}

impl Github {
    /// `GET /repos/{owner}/{repo}/contents/{path}`. A path that names a directory, a symlink or
    /// a submodule is refused.
    pub async fn get_file_contents(
        &self,
        http: &reqwest::Client,
        token: &str,
        input: &GetFileContents,
    ) -> Result<GithubFileContent, ProviderError> {
        let path = contents_path(&input.owner, &input.repo, Some(&input.path));
        let query: Vec<(&str, String)> = trimmed(input.reference.as_deref())
            .map(|reference| ("ref", reference.to_owned()))
            .into_iter()
            .collect();
        let answer: ContentsAnswer =
            send::<_, ()>(self, http, token, Method::GET, &path, &query, None).await?;
        match answer {
            ContentsAnswer::Directory(_) => Err(ProviderError::refused(
                "github",
                "path resolves to a directory, not a file",
            )),
            ContentsAnswer::Item(item) if item.entry_type != "file" => Err(ProviderError::refused(
                "github",
                "path does not resolve to a regular file",
            )),
            ContentsAnswer::Item(item) => Ok(GithubFileContent::from(*item)),
        }
    }
}
