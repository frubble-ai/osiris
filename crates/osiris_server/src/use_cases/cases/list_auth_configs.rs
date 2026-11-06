//! The providers' apps, a page at a time.

use crate::use_cases::{UseCaseResult, UseCases};
use osiris_domain::AuthConfig;
use osiris_domain::{Cursor, Page};

pub struct ListAuthConfigsArgs<'a> {
    pub cursor: &'a Cursor,
}

impl UseCases {
    pub async fn list_auth_configs(
        &self,
        args: ListAuthConfigsArgs<'_>,
    ) -> UseCaseResult<Page<AuthConfig>> {
        self.run("list_auth_configs", async {
            self.context().db.list_auth_configs(args.cursor).await
        })
        .await
    }
}
