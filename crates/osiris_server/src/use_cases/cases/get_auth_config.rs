//! One provider's app.

use crate::use_cases::{UseCaseResult, UseCases};
use osiris_domain::AuthConfig;

pub struct GetAuthConfigArgs<'a> {
    pub id: &'a str,
}

impl UseCases {
    pub async fn get_auth_config(&self, args: GetAuthConfigArgs<'_>) -> UseCaseResult<AuthConfig> {
        self.run("get_auth_config", async {
            self.context().db.auth_config(args.id).await
        })
        .await
    }
}
