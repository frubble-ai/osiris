//! `slack.list_channels`: every public and private channel the connected user can see, archived ones left out.

use crate::use_cases::{
    UseCaseResult, UseCases,
    cases::run_connected_account_action::RunConnectedAccountActionArgs,
    lib::shared::{action_args::ActionArgs, provider_of::provider_of},
};
use osiris_common::secret::{ExposeSecret, Secret};
use osiris_interfaces::ActingAs;
use osiris_providers::slack::{Slack, conversations::ListChannels, types::SlackChannel};

impl UseCases {
    /// Runs `slack.list_channels` through the connection, refused first when it lacks
    /// [`ListChannels::SCOPES`].
    pub async fn slack_list_channels(
        &self,
        args: ActionArgs<'_, ListChannels>,
    ) -> UseCaseResult<Vec<SlackChannel>> {
        self.run("slack_list_channels", async {
            let ctx = self.context();
            let provider = provider_of::<Slack>(&ctx.providers)?;
            let (http, input) = (&ctx.http, args.input);
            self.run_connected_account_action(RunConnectedAccountActionArgs {
                id: args.id,
                provider: "slack",
                action: "slack.list_channels",
                required_scopes: ListChannels::SCOPES,
                user_only: false,
                call: move |token: Secret, _acting: ActingAs| async move {
                    provider
                        .list_channels(http, token.expose_secret(), input)
                        .await
                },
            })
            .await
        })
        .await
    }
}
