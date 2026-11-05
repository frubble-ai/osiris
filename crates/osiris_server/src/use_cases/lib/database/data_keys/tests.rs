//! Each tenant's data key, wrapped by the master key.

use crate::testing::db_fixtures::{active, db};
use crate::use_cases::lib::database::Db;
use osiris_common::{crypto::MasterKey, secret::ExposeSecret};
use sqlx::PgPool;

#[sqlx::test(migrator = "crate::db::MIGRATOR")]
async fn gives_each_user_a_data_key_of_its_own(pool: PgPool) {
    let db = db(pool.clone());
    active(&db, "org_1", "s1", Some(3600)).await;
    active(&db, "org_2", "s2", Some(3600)).await;

    let tenants = sqlx::query_scalar!("SELECT tenant FROM data_key ORDER BY tenant")
        .fetch_all(&pool)
        .await
        .unwrap();
    assert_eq!(tenants, ["osiris", "user:org_1", "user:org_2"]);
    let keys = sqlx::query_scalar!("SELECT wrapped_key FROM data_key")
        .fetch_all(&pool)
        .await
        .unwrap();
    assert_ne!(keys[1], keys[2]);
}

#[sqlx::test(migrator = "crate::db::MIGRATOR")]
async fn reads_nothing_with_another_master_key(pool: PgPool) {
    let db = db(pool.clone());
    let id = active(&db, "org_1", "s1", Some(3600)).await;
    let reopened = crate::testing::db_fixtures::db(pool.clone());
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

    let stranger = Db::new(
        pool,
        MasterKey::from_secret("a-different-master-secret-long-enough").unwrap(),
        std::time::Duration::from_secs(5),
    );
    assert!(stranger.connected_account(&id).await.is_err());
}

#[sqlx::test(migrator = "crate::db::MIGRATOR")]
async fn gives_racing_first_uses_of_a_tenant_the_same_key(pool: PgPool) {
    // Several stores with nothing cached, as several instances are after a deploy, all meeting a
    // tenant for the first time at once: none fails, and they all end up with the one key stored.
    let stores: Vec<Db> = (0..8).map(|_| db(pool.clone())).collect();
    let tenant = crate::use_cases::lib::database::Tenant::user("org_race");

    let ciphers = futures::future::join_all(stores.iter().map(|db| db.cipher_for(&tenant)))
        .await
        .into_iter()
        .collect::<Result<Vec<_>, _>>()
        .unwrap();

    let sealed = ciphers[0].encrypt("the same key").unwrap();
    for cipher in &ciphers {
        assert_eq!(cipher.decrypt(&sealed).unwrap(), "the same key");
    }
    let stored = sqlx::query_scalar!(
        r#"SELECT count(*) AS "stored!" FROM data_key WHERE tenant = 'user:org_race'"#
    )
    .fetch_one(&pool)
    .await
    .unwrap();
    assert_eq!(stored, 1);
}
