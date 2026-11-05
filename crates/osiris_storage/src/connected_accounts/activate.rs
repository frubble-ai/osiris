//! The user finished the consent: saves the tokens and their first refresh task, and makes the
//! connection active, in one statement.

use super::{
    NewTokens, ScheduledRefresh,
    store_tokens::{TokenWrite, store_tokens},
};
use crate::{
    Store, Tenant,
    errors::{StorageResult, db},
};
use osiris_domain::{ConnectedAccount, ConnectionStatus};

impl Store {
    pub async fn activate(
        &self,
        account: &ConnectedAccount,
        tokens: &NewTokens,
        refresh: Option<&ScheduledRefresh>,
    ) -> StorageResult<ConnectedAccount> {
        let cipher = self.cipher_for(&Tenant::user(&account.user_id)).await?;
        let mut connection = self.pool.acquire().await.map_err(db)?;
        store_tokens(
            &mut connection,
            &cipher,
            TokenWrite {
                id: &account.id,
                tokens,
                generation: account.token_generation + 1,
                refresh,
                status: ConnectionStatus::Active,
                from: Some(ConnectionStatus::Pending),
            },
        )
        .await?;
        drop(connection);
        self.connected_account(&account.id).await
    }
}
