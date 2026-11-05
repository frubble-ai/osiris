//! The user finished the consent: saves the tokens and their first refresh task, and makes the
//! connection active, in one statement. Nothing is read back afterwards: once the write succeeds
//! there is nothing left that could fail.

use super::{
    NewTokens, ScheduledRefresh,
    store_tokens::{TokenWrite, store_tokens},
};
use crate::use_cases::UseCaseResult;
use crate::use_cases::lib::database::Db;
use crate::use_cases::lib::database::Tenant;
use crate::use_cases::lib::database::errors::db;
use osiris_domain::{ConnectedAccount, ConnectionStatus};

impl Db {
    pub async fn activate(
        &self,
        account: &ConnectedAccount,
        tokens: &NewTokens,
        refresh: Option<&ScheduledRefresh>,
    ) -> UseCaseResult<()> {
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
        Ok(())
    }
}
