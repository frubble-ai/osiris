//! `slack.list_channels`: the Slack channels the connected user can see, archived ones left out.

use crate::core::{Core, CoreResult};
use osiris_common::secret::ExposeSecret;
use osiris_providers::slack::types::SlackChannel;

impl Core {
    pub async fn slack_list_channels(&self, id: &str) -> CoreResult<Vec<SlackChannel>> {
        let (provider, http) = (&self.providers.slack, &self.http);
        self.with_token(
            id,
            "slack",
            "slack.list_channels",
            move |token| async move { provider.list_channels(http, token.expose_secret()).await },
        )
        .await
    }
}
