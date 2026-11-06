//! `slack.list_channels`: the workspace's public and private channels the user can see.

use super::{Slack, transformers::ChannelsPage, types::SlackChannel};
use crate::ProviderError;
use osiris_interfaces::call::{Request, call};
use reqwest::Method;

/// Slack's recommended largest page for `conversations.list`, a fact of its API.
const LIMIT: &str = "200";

impl Slack {
    /// Every channel, archived ones left out, across cursor pages, up to `max_items`.
    pub async fn list_channels(
        &self,
        http: &reqwest::Client,
        token: &str,
    ) -> Result<Vec<SlackChannel>, ProviderError> {
        let mut channels = Vec::new();
        let mut cursor = String::new();
        loop {
            let query = [
                ("types", "public_channel,private_channel".to_owned()),
                ("exclude_archived", "true".to_owned()),
                ("limit", LIMIT.to_owned()),
                ("cursor", cursor.clone()),
            ];
            let page: ChannelsPage = call::<ChannelsPage, ()>(
                self,
                http,
                token,
                Request {
                    method: Method::GET,
                    path: "conversations.list",
                    query: &query,
                    headers: &[],
                    body: None,
                },
            )
            .await?;
            if let Some(failure) = page.status.failure("conversations.list") {
                return Err(failure);
            }
            let next = page.next_cursor();
            channels.extend(page.channels());
            if channels.len() >= self.limits.max_items {
                channels.truncate(self.limits.max_items);
                return Ok(channels);
            }
            match next {
                Some(next) => cursor = next,
                None => return Ok(channels),
            }
        }
    }
}
