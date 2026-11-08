//! `github.download_workflow_artifact`: where an artifact's ZIP archive can be downloaded: a signed URL GitHub keeps valid for one minute.

use crate::use_cases::{
    UseCaseResult, UseCases,
    cases::{
        github::github_action_args::GithubActionArgs,
        run_connected_account_action::RunConnectedAccountActionArgs,
    },
};
use osiris_common::secret::{ExposeSecret, Secret};
use osiris_providers::github::{
    types::GithubArtifactDownload, workflows::DownloadWorkflowArtifact,
};

impl UseCases {
    /// Runs `github.download_workflow_artifact` through the connection, refused first when it lacks
    /// [`DownloadWorkflowArtifact::SCOPES`].
    pub async fn github_download_workflow_artifact(
        &self,
        args: GithubActionArgs<'_, DownloadWorkflowArtifact>,
    ) -> UseCaseResult<GithubArtifactDownload> {
        self.run("github_download_workflow_artifact", async {
            let ctx = self.context();
            let (github, http, input) = (&ctx.providers.github, &ctx.http, args.input);
            self.run_connected_account_action(RunConnectedAccountActionArgs {
                id: args.id,
                provider: "github",
                action: "github.download_workflow_artifact",
                required_scopes: DownloadWorkflowArtifact::SCOPES,
                call: move |token: Secret| async move {
                    github
                        .download_workflow_artifact(http, token.expose_secret(), input)
                        .await
                },
            })
            .await
        })
        .await
    }
}
