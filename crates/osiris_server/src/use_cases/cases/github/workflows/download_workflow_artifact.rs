//! `github.download_workflow_artifact`: where an artifact's ZIP archive can be downloaded: a signed URL GitHub keeps valid for one minute.

use crate::use_cases::{
    UseCaseResult, UseCases,
    cases::run_connected_account_action::RunConnectedAccountActionArgs,
    lib::shared::{action_args::ActionArgs, provider_of::provider_of},
};
use osiris_common::secret::{ExposeSecret, Secret};
use osiris_interfaces::ActingAs;
use osiris_providers::github::{
    Github, types::GithubArtifactDownload, workflows::DownloadWorkflowArtifact,
};

impl UseCases {
    /// Runs `github.download_workflow_artifact` through the connection, refused first when it lacks
    /// [`DownloadWorkflowArtifact::SCOPES`].
    pub async fn github_download_workflow_artifact(
        &self,
        args: ActionArgs<'_, DownloadWorkflowArtifact>,
    ) -> UseCaseResult<GithubArtifactDownload> {
        self.run("github_download_workflow_artifact", async {
            let ctx = self.context();
            let provider = provider_of::<Github>(&ctx.providers)?;
            let (http, input) = (&ctx.http, args.input);
            self.run_connected_account_action(RunConnectedAccountActionArgs {
                id: args.id,
                provider: "github",
                action: "github.download_workflow_artifact",
                required_scopes: DownloadWorkflowArtifact::SCOPES,
                user_only: false,
                call: move |token: Secret, _acting: ActingAs| async move {
                    provider
                        .download_workflow_artifact(http, token.expose_secret(), input)
                        .await
                },
            })
            .await
        })
        .await
    }
}
