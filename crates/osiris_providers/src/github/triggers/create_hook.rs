//! A trigger's repository webhook, created at GitHub, or, when one already delivers to the
//! trigger's URL (GitHub answers 422), that one aligned to what the trigger asks for.

use super::hook_body::hook_body;
use crate::{
    ProviderError,
    github::{Github, path_segment::path_segment, request::send},
};
use reqwest::Method;
use serde::Deserialize;
use serde_json::Value;

#[derive(Deserialize)]
struct CreatedHook {
    id: u64,
}

impl Github {
    /// The id of `owner/repo`'s webhook delivering `events` to `callback_url`, signed with
    /// `secret`.
    #[allow(
        clippy::too_many_arguments,
        reason = "a webhook's parts, as GitHub takes them"
    )]
    pub async fn create_hook(
        &self,
        http: &reqwest::Client,
        token: &str,
        owner: &str,
        repo: &str,
        callback_url: &str,
        secret: &str,
        events: &[String],
        insecure_ssl: bool,
    ) -> Result<String, ProviderError> {
        let path = format!(
            "/repos/{}/{}/hooks",
            path_segment(owner),
            path_segment(repo)
        );
        let body = hook_body(callback_url, secret, events, insecure_ssl);
        match send::<CreatedHook, Value>(self, http, token, Method::POST, &path, &[], Some(&body))
            .await
        {
            Ok(created) => Ok(created.id.to_string()),
            Err(error) if error.status() == Some(422) => {
                let Some(id) = self
                    .find_hook(http, token, owner, repo, callback_url)
                    .await?
                else {
                    return Err(ProviderError::refused_with_status(
                        "github",
                        422,
                        "GitHub refused the webhook's configuration",
                    ));
                };
                let existing = format!("{path}/{}", path_segment(&id));
                send::<Value, Value>(
                    self,
                    http,
                    token,
                    Method::PATCH,
                    &existing,
                    &[],
                    Some(&body),
                )
                .await?;
                Ok(id)
            }
            Err(error) => Err(error),
        }
    }
}
