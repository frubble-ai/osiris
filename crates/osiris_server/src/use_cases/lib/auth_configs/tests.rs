//! `auth_config` against a real Postgres: each test gets a fresh database with the migrations.

use super::AuthConfigChanges;
use crate::testing::db_fixtures::{db, github_config, pending};
use osiris_common::secret::ExposeSecret;
use osiris_domain::Cursor;
use sqlx::PgPool;

#[sqlx::test(migrator = "crate::db::MIGRATOR")]
async fn stores_the_client_secret_encrypted_and_reads_it_back(pool: PgPool) {
    let db = db(pool.clone());
    let id = github_config(&db).await;

    let raw = sqlx::query_scalar!("SELECT client_secret FROM auth_config WHERE id = $1", id)
        .fetch_one(&pool)
        .await
        .unwrap();
    assert!(!String::from_utf8_lossy(&raw).contains("client-secret"));
    assert_eq!(
        db.auth_config(&id)
            .await
            .unwrap()
            .client_secret
            .expose_secret(),
        "client-secret"
    );

    let updated = db
        .update_auth_config(
            &id,
            AuthConfigChanges {
                scopes: Some(vec!["repo".into()]),
                ..Default::default()
            },
        )
        .await
        .unwrap();
    assert_eq!(updated.scopes, vec!["repo".to_owned()]);
    assert_eq!(updated.client_secret.expose_secret(), "client-secret");
}

#[sqlx::test(migrator = "crate::db::MIGRATOR")]
async fn refuses_to_delete_an_auth_config_in_use(pool: PgPool) {
    let db = db(pool);
    let id = github_config(&db).await;
    let account = pending(&db, &id, "org_1", "state-1").await;

    assert!(db.delete_auth_config(&id).await.is_err());
    db.delete_connected_account(&account).await.unwrap();
    db.delete_auth_config(&id).await.unwrap();
}

#[sqlx::test(migrator = "crate::db::MIGRATOR")]
async fn pages_forwards_and_back(pool: PgPool) {
    let db = db(pool);
    for _ in 0..5 {
        github_config(&db).await;
    }
    let first = db
        .list_auth_configs(&Cursor {
            limit: 2,
            ..Default::default()
        })
        .await
        .unwrap();
    assert!(first.previous.is_none());
    let second = db
        .list_auth_configs(&Cursor {
            after: first.next.clone(),
            limit: 2,
            ..Default::default()
        })
        .await
        .unwrap();
    assert_eq!(second.items.len(), 2);
    let back = db
        .list_auth_configs(&Cursor {
            before: second.previous.clone(),
            limit: 2,
            ..Default::default()
        })
        .await
        .unwrap();
    let ids = |page: &osiris_domain::Page<osiris_domain::AuthConfig>| {
        page.items.iter().map(|c| c.id.clone()).collect::<Vec<_>>()
    };
    assert_eq!(ids(&back), ids(&first));
}
