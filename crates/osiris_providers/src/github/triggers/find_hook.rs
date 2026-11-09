//! A repository's webhook already delivering to a URL: what GitHub refuses to create twice.

use crate::{
    ProviderError,
    github::{Github, paging::paging, path_segment::path_segment, request::send},
};
use reqwest::Method;
use serde::Deserialize;

/// GitHub's largest page, a fact of its API.
const PER_PAGE: u32 = 100;

#[derive(Deserialize)]
struct HookAnswer {
    id: u64,
    #[serde(default)]
    config: HookConfigAnswer,
}

#[derive(Deserialize, Default)]
struct HookConfigAnswer {
    url: Option<String>,
}

impl Github {
    /// The id of `owner/repo`'s webhook delivering to `callback_url`, if it has one; none when the
    /// repository isn't there.
    pub async fn find_hook(
        &self,
        http: &reqwest::Client,
        token: &str,
        owner: &str,
        repo: &str,
        callback_url: &str,
    ) -> Result<Option<String>, ProviderError> {
        let path = format!(
            "/repos/{}/{}/hooks",
            path_segment(owner),
            path_segment(repo)
        );
        // At most `[providers.listing] max_scan_pages` pages are read looking for it.
        for page in 1..=self.limits.max_scan_pages {
            let hooks: Vec<HookAnswer> = match send::<_, ()>(
                self,
                http,
                token,
                Method::GET,
                &path,
                &paging(Some(PER_PAGE), Some(page)),
                None,
            )
            .await
            {
                Ok(hooks) => hooks,
                Err(error) if error.is_not_found() => return Ok(None),
                Err(error) => return Err(error),
            };
            if let Some(hook) = hooks
                .iter()
                .find(|hook| hook.config.url.as_deref() == Some(callback_url))
            {
                return Ok(Some(hook.id.to_string()));
            }
            if hooks.len() < usize::try_from(PER_PAGE).unwrap_or(usize::MAX) {
                return Ok(None);
            }
        }
        Ok(None)
    }
}
