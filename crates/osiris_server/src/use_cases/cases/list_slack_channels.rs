//! `slack.list_channels`: the Slack channels the connected user can see, archived ones left out.

use crate::use_cases::{
    UseCaseResult, UseCases, cases::run_connection_action::RunConnectionActionArgs,
};
use osiris_common::secret::{ExposeSecret, Secret};
use osiris_providers::slack::types::SlackChannel;

pub struct ListSlackChannelsArgs<'a> {
    /// The connection to read through.
    pub id: &'a str,
}

impl UseCases {
    pub async fn list_slack_channels(
        &self,
        args: ListSlackChannelsArgs<'_>,
    ) -> UseCaseResult<Vec<SlackChannel>> {
        self.run("list_slack_channels", async {
            let ctx = self.context();
            let (provider, http) = (&ctx.providers.slack, &ctx.http);
            self.run_connection_action(RunConnectionActionArgs {
                id: args.id,
                provider: "slack",
                action: "slack.list_channels",
                call: move |token: Secret| async move {
                    provider.list_channels(http, token.expose_secret()).await
                },
            })
            .await
        })
        .await
    }
}
