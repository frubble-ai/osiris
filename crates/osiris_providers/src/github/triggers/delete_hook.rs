//! A trigger's repository webhook, deleted at GitHub. One already gone counts as deleted.

use crate::{
    ProviderError,
    github::{Github, path_segment::path_segment, request::send},
};
use reqwest::Method;

impl Github {
    /// `DELETE /repos/{owner}/{repo}/hooks/{hook_id}`; a 404 is a webhook already gone.
    pub async fn delete_hook(
        &self,
        http: &reqwest::Client,
        token: &str,
        owner: &str,
        repo: &str,
        hook_id: &str,
    ) -> Result<(), ProviderError> {
        let path = format!(
            "/repos/{}/{}/hooks/{}",
            path_segment(owner),
            path_segment(repo),
            path_segment(hook_id)
        );
        match send::<serde_json::Value, ()>(self, http, token, Method::DELETE, &path, &[], None)
            .await
        {
            Ok(_) => Ok(()),
            Err(error) if error.status() == Some(404) => Ok(()),
            Err(error) => Err(error),
        }
    }
}
