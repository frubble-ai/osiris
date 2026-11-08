//! Runs an action with a connection's token. A rejected token is refreshed once and the action run
//! again; rejected still, or with nothing to refresh it with, the grant is gone and the connection
//! is revoked. An installation connection's rejected token is minted again instead, and tried once.

use crate::use_cases::{
    UseCaseResult, UseCases,
    cases::{
        connected_account_token::ConnectedAccountTokenArgs,
        get_connected_account::GetConnectedAccountArgs,
        refresh_connected_account::RefreshConnectedAccountArgs,
        revoke_connected_account::RevokeConnectedAccountArgs,
    },
    lib::{refresh::When, shared::domain_errors::provider},
};
use osiris_common::secret::Secret;
use osiris_domain::{ConnectionStatus, CredentialKind, DomainError};
use osiris_interfaces::ProviderError;

pub struct RunConnectedAccountActionArgs<'a, F> {
    pub id: &'a str,
    pub provider: &'a str,
    pub action: &'a str,
    /// The scopes the action needs, checked before the call.
    pub required_scopes: &'a [&'a str],
    /// The provider call, given the token.
    pub call: F,
}

impl UseCases {
    pub async fn run_connected_account_action<T, F, Fut>(
        &self,
        args: RunConnectedAccountActionArgs<'_, F>,
    ) -> UseCaseResult<T>
    where
        F: Fn(Secret) -> Fut,
        Fut: Future<Output = Result<T, ProviderError>>,
    {
        self.run("run_connected_account_action", async {
            let RunConnectedAccountActionArgs {
                id,
                provider: provider_id,
                action,
                required_scopes,
                call,
            } = args;
            let token = self
                .connected_account_token(ConnectedAccountTokenArgs {
                    id,
                    provider: provider_id,
                    action,
                    required_scopes,
                })
                .await?;
            let first = match call(token).await {
                Ok(answer) => return Ok(answer),
                Err(ProviderError::Unauthorized { message, .. }) => message,
                Err(error) => return Err(provider(error)),
            };
            let current = self
                .get_connected_account(GetConnectedAccountArgs { id })
                .await?;
            if current.credential_kind == CredentialKind::AppInstallation {
                // The installation's token was refused: a new one is minted and tried once. Refused
                // still, the app no longer acts as the installation (uninstalled or suspended).
                if let Some(installation_id) = current.installation_id.as_deref() {
                    self.context()
                        .installation_tokens
                        .forget(&current.auth_config_id, installation_id);
                }
                let token = self
                    .connected_account_token(ConnectedAccountTokenArgs {
                        id,
                        provider: provider_id,
                        action,
                        required_scopes,
                    })
                    .await?;
                return match call(token).await {
                    Ok(answer) => Ok(answer),
                    Err(ProviderError::Unauthorized { message, .. }) => {
                        self.revoke_connected_account(RevokeConnectedAccountArgs {
                            id,
                            reason: &message,
                        })
                        .await?;
                        Err(DomainError::NotActive("expired").into())
                    }
                    Err(error) => Err(provider(error)),
                };
            }
            let account = self
                .refresh_connected_account(RefreshConnectedAccountArgs {
                    id,
                    when: When::Now,
                    running: None,
                })
                .await?
                .into_account()
                .ok_or(DomainError::Internal)?;
            if account.status != ConnectionStatus::Active {
                return Err(DomainError::NotActive(account.status.as_str()).into());
            }
            let Some(tokens) = account
                .tokens
                .filter(|tokens| tokens.refresh_token.is_some())
            else {
                self.revoke_connected_account(RevokeConnectedAccountArgs { id, reason: &first })
                    .await?;
                return Err(DomainError::NotActive("expired").into());
            };
            match call(tokens.access_token).await {
                Ok(answer) => Ok(answer),
                Err(ProviderError::Unauthorized { message, .. }) => {
                    self.revoke_connected_account(RevokeConnectedAccountArgs {
                        id,
                        reason: &message,
                    })
                    .await?;
                    Err(DomainError::NotActive("expired").into())
                }
                Err(error) => Err(provider(error)),
            }
        })
        .await
    }
}
