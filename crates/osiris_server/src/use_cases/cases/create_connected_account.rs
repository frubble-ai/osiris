//! Creates a connection: as an installation of the auth config's app when an installation is
//! named (active at once, no consent), or else through the user's consent, which needs where to
//! send the browser afterwards.

use crate::use_cases::{
    UseCaseResult, UseCases,
    cases::{connect_installation::ConnectInstallationArgs, start_connect::StartConnectArgs},
};
use osiris_domain::{ConnectedAccount, DomainError};

pub struct CreateConnectedAccountArgs {
    pub auth_config_id: String,
    pub user_id: String,
    pub alias: Option<String>,
    /// Where the browser goes after a consent; required unless `installation_id` is given.
    pub callback_url: Option<String>,
    /// The provider's id of the installation to act as, instead of a consent.
    pub installation_id: Option<String>,
}

/// A connection created: the consent's URL to send the browser to, unless it is an installation.
pub struct ConnectionCreated {
    pub account: ConnectedAccount,
    pub redirect_url: Option<String>,
}

impl UseCases {
    pub async fn create_connected_account(
        &self,
        args: CreateConnectedAccountArgs,
    ) -> UseCaseResult<ConnectionCreated> {
        self.run("create_connected_account", async {
            if let Some(installation_id) = args.installation_id {
                let account = self
                    .connect_installation(ConnectInstallationArgs {
                        auth_config_id: args.auth_config_id,
                        user_id: args.user_id,
                        alias: args.alias,
                        installation_id,
                    })
                    .await?;
                return Ok(ConnectionCreated {
                    account,
                    redirect_url: None,
                });
            }
            let callback_url = args.callback_url.ok_or_else(|| {
                DomainError::Invalid("callback_url is required for a consent".into())
            })?;
            let (account, redirect_url) = self
                .start_connect(StartConnectArgs {
                    auth_config_id: args.auth_config_id,
                    user_id: args.user_id,
                    alias: args.alias,
                    callback_url,
                })
                .await?;
            Ok(ConnectionCreated {
                account,
                redirect_url: Some(redirect_url),
            })
        })
        .await
    }
}
