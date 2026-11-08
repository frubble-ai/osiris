//! What the database tests share: a database on the test's own pool, and its first rows.

#![allow(dead_code, reason = "each test file uses some of these")]
#![allow(
    clippy::unwrap_used,
    clippy::indexing_slicing,
    reason = "test fixtures: a failure here is the test failing"
)]

use crate::db::register_providers::register_providers;
use crate::use_cases::lib::{
    auth_configs::NewAuthConfig,
    connected_accounts::{NewPending, NewTokens, ScheduledRefresh},
    database::Db,
};
use fake::Fake;
use osiris_common::{crypto::MasterKey, secret::secret};
use osiris_providers::mocks::fake::FAKE_ID;
use sqlx::PgPool;
use time::{Duration, OffsetDateTime};

pub const MASTER_SECRET: &str = "a-test-secret-that-is-long-enough-for-osiris";

pub fn db(pool: PgPool) -> Db {
    Db::new(
        pool,
        MasterKey::from_secret(MASTER_SECRET).unwrap(),
        std::time::Duration::from_secs(5),
    )
}

/// A refresh task as a use case names it, due in an hour.
pub fn refresh_task(name: &str) -> ScheduledRefresh {
    ScheduledRefresh {
        name: name.into(),
        at: OffsetDateTime::now_utc() + Duration::hours(1),
    }
}

/// A random value for a secret, so a test reads what it expects from what it stored.
pub fn generate_secret(prefix: &str) -> String {
    format!("{prefix}-{}", 24.fake::<String>())
}

/// A random tenant id for arranging connections.
pub fn generate_user_id() -> String {
    generate_secret("org")
}

/// A random OAuth `state` for arranging a consent.
pub fn generate_state() -> String {
    generate_secret("state")
}

/// The fake provider's app, saved; answers its id.
pub async fn fake_config(db: &Db) -> String {
    fake_config_with(db, &generate_secret("client-secret")).await
}

/// The fake provider's app with `client_secret`, saved with the provider's row; answers its id.
pub async fn fake_config_with(db: &Db, client_secret: &str) -> String {
    // The migrations write only the registered providers' rows; the fake one is added here.
    register_providers(&mut db.pool.acquire().await.unwrap(), &[FAKE_ID])
        .await
        .unwrap();
    db.create_auth_config(NewAuthConfig {
        provider: FAKE_ID.into(),
        client_id: generate_secret("client"),
        client_secret: secret(client_secret),
        scopes: vec![],
        settings: serde_json::json!({}),
        secret_settings: None,
    })
    .await
    .unwrap()
    .id
}

pub async fn pending(db: &Db, auth_config_id: &str, user_id: &str, state: &str) -> String {
    pending_with(
        db,
        auth_config_id,
        user_id,
        state,
        &generate_secret("verifier"),
    )
    .await
}

pub async fn pending_with(
    db: &Db,
    auth_config_id: &str,
    user_id: &str,
    state: &str,
    pkce_verifier: &str,
) -> String {
    db.create_pending(NewPending {
        auth_config_id: auth_config_id.into(),
        user_id: user_id.into(),
        alias: Some("attempt-1".into()),
        callback_url: "http://localhost:8080/integrations/callback".into(),
        oauth_state: state.into(),
        oauth_state_expires_at: OffsetDateTime::now_utc() + Duration::minutes(15),
        pkce_verifier: Some(secret(pkce_verifier)),
    })
    .await
    .unwrap()
    .id
}

pub fn generate_tokens(expires_in: Option<i64>) -> NewTokens {
    NewTokens {
        access_token: secret(generate_secret("access")),
        refresh_token: Some(secret(generate_secret("refresh"))),
        access_token_expires_at: expires_in
            .map(|seconds| OffsetDateTime::now_utc() + Duration::seconds(seconds)),
        scope: Some("read".into()),
    }
}

pub fn no_expiry(access: &str) -> NewTokens {
    NewTokens {
        access_token: secret(access),
        refresh_token: None,
        access_token_expires_at: None,
        scope: None,
    }
}

/// An active connection with tokens expiring in `expires_in` seconds, and its refresh task.
pub async fn active(db: &Db, user_id: &str, state: &str, expires_in: Option<i64>) -> String {
    active_with(db, user_id, state, &generate_tokens(expires_in)).await
}

/// An active connection holding `tokens`, and its refresh task.
pub async fn active_with(db: &Db, user_id: &str, state: &str, tokens: &NewTokens) -> String {
    let config = fake_config(db).await;
    let id = pending(db, &config, user_id, state).await;
    let account = db.connected_account(&id).await.unwrap();
    db.activate(
        &account,
        tokens,
        Some(&refresh_task(&generate_secret("refresh"))),
    )
    .await
    .unwrap();
    id
}
