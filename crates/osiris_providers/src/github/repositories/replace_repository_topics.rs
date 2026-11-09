//! `github.replace_repository_topics`: a repository's topics replaced by a new set.

use crate::{
    ProviderError,
    github::{
        Github, path_segment::path_segment, request::send, transformers::topics::TopicsAnswer,
        types::GithubTopics,
    },
};
use reqwest::Method;
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

#[derive(Debug, Deserialize, ToSchema)]
#[serde(deny_unknown_fields)]
pub struct ReplaceRepositoryTopics {
    pub owner: String,
    pub repo: String,
    /// Every topic the repository has from now on, in lowercase; an empty list removes them all.
    pub names: Vec<String>,
}

impl ReplaceRepositoryTopics {
    /// The scopes it needs, as `OpenConnector` names them.
    pub const SCOPES: &[&str] = &["repo"];
}

#[derive(Serialize)]
struct Body<'a> {
    names: &'a [String],
}

impl Github {
    /// `PUT /repos/{owner}/{repo}/topics`.
    pub async fn replace_repository_topics(
        &self,
        http: &reqwest::Client,
        token: &str,
        input: &ReplaceRepositoryTopics,
    ) -> Result<GithubTopics, ProviderError> {
        let path = format!(
            "/repos/{}/{}/topics",
            path_segment(&input.owner),
            path_segment(&input.repo)
        );
        let body = Body {
            names: &input.names,
        };
        let answer: TopicsAnswer =
            send(self, http, token, Method::PUT, &path, &[], Some(&body)).await?;
        Ok(answer.into())
    }
}
