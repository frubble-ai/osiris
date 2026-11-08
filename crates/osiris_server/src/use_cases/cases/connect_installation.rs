//! Connects as an app's installation: no consent, the app's own credentials (its auth config's
//! settings) mint a token for the installation, which proves it exists and the app may act as it.
//! The connection is active at once, holds no tokens, and `connection.active` is published as
//! when a consent finishes.

use crate::use_cases::lib::{
    connected_accounts::create_installation::NewInstallation, events::publish::publish,
    installation_tokens::installation_token::installation_token,
};
use crate::use_cases::{UseCaseResult, UseCases};
use error_stack::ResultExt;
use osiris_domain::{ConnectedAccount, DomainError, Event, EventType};

pub struct ConnectInstallationArgs {
    pub auth_config_id: String,
    pub user_id: String,
    pub alias: Option<String>,
    /// The provider's id of the installation to act as.
    pub installation_id: String,
}

impl UseCases {
    /// The active installation connection.
    pub async fn connect_installation(
        &self,
        args: ConnectInstallationArgs,
    ) -> UseCaseResult<ConnectedAccount> {
        self.run("connect_installation", async {
            let ctx = self.context();
            if args.user_id.trim().is_empty() {
                return Err(DomainError::Invalid("user_id is required".into()).into());
            }
            if args.installation_id.trim().is_empty() {
                return Err(DomainError::Invalid("installation_id cannot be blank".into()).into());
            }
            let config = ctx.db.auth_config(&args.auth_config_id).await?;
            let implementation = ctx
                .providers
                .get(&config.provider)
                .ok_or(DomainError::Internal)
                .attach_with(|| format!("no provider {:?}", config.provider))?;
            // Minting one proves the installation exists and the app may act as it.
            installation_token(
                implementation,
                &ctx.http,
                &ctx.installation_tokens,
                ctx.config.settings.providers.installation_tokens.margin(),
                &config,
                &args.installation_id,
            )
            .await?;
            let account = ctx
                .db
                .create_installation(NewInstallation {
                    auth_config_id: config.id,
                    user_id: args.user_id,
                    alias: args.alias,
                    installation_id: args.installation_id,
                })
                .await?;
            publish(
                &ctx.messages,
                ctx.scheduling(),
                &Event::new(EventType::ConnectionActive, &account.id),
            )
            .await;
            tracing::info!(connected_account_id = %account.id, "installation connection active");
            Ok(account)
        })
        .await
    }
}
