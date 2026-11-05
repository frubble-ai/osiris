//! The providers' apps, a page at a time.

use crate::use_cases::{UseCaseResult, UseCases};
use osiris_common::errors::ReportSwitchExt;
use osiris_domain::AuthConfig;
use osiris_storage::{Cursor, Page};

pub struct ListAuthConfigsArgs<'a> {
    pub cursor: &'a Cursor,
}

impl UseCases {
    pub async fn list_auth_configs(
        &self,
        args: ListAuthConfigsArgs<'_>,
    ) -> UseCaseResult<Page<AuthConfig>> {
        self.run("list_auth_configs", async {
            self.context()
                .store
                .list_auth_configs(args.cursor)
                .await
                .switch()
        })
        .await
    }
}
