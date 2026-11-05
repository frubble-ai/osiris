//! Deletes an app no connection uses; one still in use is a conflict.

use crate::use_cases::{UseCaseResult, UseCases};
use osiris_common::errors::ReportSwitchExt;

pub struct DeleteAuthConfigArgs<'a> {
    pub id: &'a str,
}

impl UseCases {
    pub async fn delete_auth_config(&self, args: DeleteAuthConfigArgs<'_>) -> UseCaseResult<()> {
        self.run("delete_auth_config", async {
            self.context()
                .store
                .delete_auth_config(args.id)
                .await
                .switch()
        })
        .await
    }
}
