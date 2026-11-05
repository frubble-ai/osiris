//! What the storage tests share: a store on the test's own database, and its first rows.

#![allow(dead_code, reason = "each test file uses some of these")]
#![allow(
    clippy::unwrap_used,
    clippy::indexing_slicing,
    reason = "test fixtures: a failure here is the test failing"
)]

use osiris_common::{crypto::MasterKey, secret::secret};
use osiris_storage::{
    Store,
    auth_configs::NewAuthConfig,
    connected_accounts::{NewPending, NewTokens, ScheduledRefresh},
};
use sqlx::PgPool;
use time::{Duration, OffsetDateTime};

pub const MASTER_SECRET: &str = "a-test-secret-that-is-long-enough-for-osiris";

pub fn store(pool: PgPool) -> Store {
    Store::new(
        pool,
        MasterKey::from_secret(MASTER_SECRET).unwrap(),
        std::time::Duration::from_secs(5),
    )
}

/// A refresh task as the core would name it, due in an hour.
pub fn refresh_task(name: &str) -> ScheduledRefresh {
    ScheduledRefresh {
        name: name.into(),
        at: OffsetDateTime::now_utc() + Duration::hours(1),
    }
}

pub async fn github_config(store: &Store) -> String {
    store
        .create_auth_config(NewAuthConfig {
            provider: "github".into(),
            client_id: "Iv1.client".into(),
            client_secret: secret("client-secret"),
            scopes: vec![],
        })
        .await
        .unwrap()
        .id
}

pub async fn pending(store: &Store, auth_config_id: &str, user_id: &str, state: &str) -> String {
    store
        .create_pending(NewPending {
            auth_config_id: auth_config_id.into(),
            user_id: user_id.into(),
            alias: Some("attempt-1".into()),
            callback_url: "http://localhost:8080/integrations/callback".into(),
            oauth_state: state.into(),
            oauth_state_expires_at: OffsetDateTime::now_utc() + Duration::minutes(15),
            pkce_verifier: Some(secret("verifier")),
        })
        .await
        .unwrap()
        .id
}

pub fn tokens(expires_in: Option<i64>) -> NewTokens {
    NewTokens {
        access_token: secret("access-1"),
        refresh_token: Some(secret("refresh-1")),
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
pub async fn active(store: &Store, user_id: &str, state: &str, expires_in: Option<i64>) -> String {
    let config = github_config(store).await;
    let id = pending(store, &config, user_id, state).await;
    let account = store.connected_account(&id).await.unwrap();
    store
        .activate(
            &account,
            &tokens(expires_in),
            Some(&refresh_task("refresh-1")),
        )
        .await
        .unwrap();
    id
}
