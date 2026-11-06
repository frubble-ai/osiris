//! The provider permanently rejects the connection's token, which means the grant no longer exists
//! there. The connection is marked as expired, a `connection.revoked` event is published, its
//! refresh task is cancelled. The caller answers that the user must reconnect.

use crate::use_cases::{
    UseCaseResult, UseCases,
    lib::{events::publish::publish, tasks::cancel_quietly::cancel_quietly},
};

pub struct RevokeConnectedAccountArgs<'a> {
    pub id: &'a str,
    /// The provider's own text, for the log only.
    pub reason: &'a str,
}

impl UseCases {
    pub async fn revoke_connected_account(
        &self,
        args: RevokeConnectedAccountArgs<'_>,
    ) -> UseCaseResult<()> {
        self.run("revoke_connected_account", async {
            let ctx = self.context();
            let RevokeConnectedAccountArgs { id, reason } = args;
            // The provider's own text stays in the log; the connection stores a fixed reason.
            tracing::warn!(connected_account_id = id, %reason, "the provider rejects the token; the grant is gone");
            let lock = ctx.db.lock_for_refresh(id).await?;
            let task = lock.account.refresh_task.clone();
            let (event, _) = lock
                .revoke("the provider no longer accepts the connection's token; reconnect")
                .await?;
            publish(&ctx.messages, ctx.scheduling(), &event).await;
            if let Some(task) = task {
                cancel_quietly(ctx.scheduling(), &task).await;
            }
            Ok(())
        })
        .await
    }
}
