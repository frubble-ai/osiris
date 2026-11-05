//! Run an action with the connection's token. A rejected token is refreshed once and the action
//! run again; rejected still, or with nothing to refresh it with, the grant is gone.

use crate::core::{Core, CoreResult, errors::provider, refresh::When};
use osiris_common::secret::Secret;
use osiris_domain::{ConnectionStatus, DomainError};
use osiris_interfaces::ProviderError;

impl Core {
    pub(crate) async fn with_token<T, F, Fut>(
        &self,
        id: &str,
        provider_id: &str,
        action: &str,
        run: F,
    ) -> CoreResult<T>
    where
        F: Fn(Secret) -> Fut,
        Fut: Future<Output = Result<T, ProviderError>>,
    {
        let token = self.token_for(id, provider_id, action).await?;
        let first = match run(token).await {
            Ok(answer) => return Ok(answer),
            Err(ProviderError::Unauthorized { message, .. }) => message,
            Err(error) => return Err(provider(error)),
        };
        let account = self
            .refresh(id, When::Now, None)
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
            return self.revoked(id, &first).await;
        };
        match run(tokens.access_token).await {
            Ok(answer) => Ok(answer),
            Err(ProviderError::Unauthorized { message, .. }) => self.revoked(id, &message).await,
            Err(error) => Err(provider(error)),
        }
    }
}
