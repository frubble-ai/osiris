//! Each tenant's data key, wrapped by the master key.

#[path = "support/fixtures.rs"]
mod fixtures;

use fixtures::{active, store};
use osiris_common::{crypto::MasterKey, secret::ExposeSecret};
use osiris_storage::Store;
use sqlx::PgPool;

#[sqlx::test(migrator = "osiris_storage::MIGRATOR")]
async fn gives_each_user_a_data_key_of_its_own(pool: PgPool) {
    let store = store(pool.clone());
    active(&store, "org_1", "s1", Some(3600)).await;
    active(&store, "org_2", "s2", Some(3600)).await;

    let tenants: Vec<String> = sqlx::query_scalar("SELECT tenant FROM data_key ORDER BY tenant")
        .fetch_all(&pool)
        .await
        .unwrap();
    assert_eq!(tenants, ["osiris", "user:org_1", "user:org_2"]);
    let keys: Vec<Vec<u8>> = sqlx::query_scalar("SELECT wrapped_key FROM data_key")
        .fetch_all(&pool)
        .await
        .unwrap();
    assert_ne!(keys[1], keys[2]);
}

#[sqlx::test(migrator = "osiris_storage::MIGRATOR")]
async fn reads_nothing_with_another_master_key(pool: PgPool) {
    let store = store(pool.clone());
    let id = active(&store, "org_1", "s1", Some(3600)).await;
    let reopened = fixtures::store(pool.clone());
    assert_eq!(
        reopened
            .connected_account(&id)
            .await
            .unwrap()
            .tokens
            .unwrap()
            .access_token
            .expose_secret(),
        "access-1"
    );

    let stranger = Store::new(
        pool,
        MasterKey::from_secret("a-different-master-secret-long-enough").unwrap(),
        std::time::Duration::from_secs(5),
    );
    assert!(stranger.connected_account(&id).await.is_err());
}
