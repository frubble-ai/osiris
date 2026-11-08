//! `fake.whoami` run through the action runner, as a provider's action use case runs its call.

#![allow(
    clippy::unwrap_used,
    reason = "tests: a failure here is the test failing"
)]

use crate::testing::TestContext;
use crate::use_cases::{
    UseCaseResult, cases::run_connected_account_action::RunConnectedAccountActionArgs,
    lib::shared::provider_of::provider_of,
};
use osiris_common::secret::{ExposeSecret, Secret};
use osiris_interfaces::ActingAs;
use osiris_providers::mocks::fake::{FAKE_ID, FakeProvider};
use serde_json::Value;

/// What the action asks for, besides the connection.
pub struct Whoami<'a> {
    pub provider: &'a str,
    pub required_scopes: &'a [&'a str],
    pub user_only: bool,
}

/// The fake provider's action, on any of its connections.
pub const PLAIN: Whoami<'static> = Whoami {
    provider: FAKE_ID,
    required_scopes: &[],
    user_only: false,
};

/// Runs `fake.whoami` through the connection `id`.
pub async fn run_whoami(app: &TestContext, id: &str, whoami: Whoami<'_>) -> UseCaseResult<Value> {
    let ctx = app.use_cases.context();
    let fake = provider_of::<FakeProvider>(&ctx.providers).unwrap();
    let http = &ctx.http;
    app.use_cases
        .run_connected_account_action(RunConnectedAccountActionArgs {
            id,
            provider: whoami.provider,
            action: "fake.whoami",
            required_scopes: whoami.required_scopes,
            user_only: whoami.user_only,
            call: move |token: Secret, _acting: ActingAs| async move {
                fake.whoami(http, token.expose_secret()).await
            },
        })
        .await
}
