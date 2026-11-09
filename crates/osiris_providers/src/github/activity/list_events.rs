//! A page of any of GitHub's event feeds, which all answer the same way.

use crate::{
    ProviderError,
    github::{
        Github, paging::paging, request::send, transformers::event::EventAnswer, types::GithubEvent,
    },
};
use reqwest::Method;

/// The page `per_page` and `page` name of the feed at `path`, already encoded.
pub(super) async fn list_events(
    github: &Github,
    http: &reqwest::Client,
    token: &str,
    path: &str,
    per_page: Option<u32>,
    page: Option<u32>,
) -> Result<Vec<GithubEvent>, ProviderError> {
    let query = paging(per_page, page);
    let answer: Vec<EventAnswer> =
        send::<_, ()>(github, http, token, Method::GET, path, &query, None).await?;
    Ok(answer.into_iter().map(GithubEvent::from).collect())
}
