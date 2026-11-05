//! `connected_account`: the consent, tokens saved with their refresh task, the token generation,
//! refresh locking, and the completeness rule.

use crate::testing::db_fixtures::{
    active, db, github_config, no_expiry, pending, refresh_task, tokens,
};
use osiris_common::secret::ExposeSecret;
use osiris_domain::{AccountFilter, Cursor};
use osiris_domain::{ConnectionStatus, EventType};
use sqlx::PgPool;

#[sqlx::test(migrator = "crate::db::MIGRATOR")]
async fn lets_a_consent_be_claimed_once(pool: PgPool) {
    let db = db(pool);
    let config = github_config(&db).await;
    let id = pending(&db, &config, "org_1", "state-1").await;

    let claimed = db.claim_consent("state-1").await.unwrap().unwrap();
    assert_eq!(claimed.account.id, id);
    assert_eq!(
        claimed
            .pkce_verifier
            .as_ref()
            .map(ExposeSecret::expose_secret),
        Some("verifier")
    );
    assert!(!claimed.expired);
    assert!(db.claim_consent("state-1").await.unwrap().is_none());
}

#[sqlx::test(migrator = "crate::db::MIGRATOR")]
async fn saves_the_tokens_with_their_refresh_task_as_generation_one(pool: PgPool) {
    let db = db(pool);
    let id = active(&db, "org_1", "state-1", Some(3600)).await;

    let account = db.connected_account(&id).await.unwrap();
    assert_eq!(account.status, ConnectionStatus::Active);
    assert_eq!(account.token_generation, 1);
    assert_eq!(account.refresh_task.as_deref(), Some("refresh-1"));
    assert!(account.refresh_task_at.is_some());
    assert!(account.is_complete());
    assert_eq!(
        account.tokens.unwrap().access_token.expose_secret(),
        "access-1"
    );
}

#[sqlx::test(migrator = "crate::db::MIGRATOR")]
async fn refuses_to_save_a_refreshable_token_without_its_refresh_task(pool: PgPool) {
    let db = db(pool);
    let config = github_config(&db).await;
    let id = pending(&db, &config, "org_1", "state-1").await;
    let account = db.connected_account(&id).await.unwrap();

    // The database's completeness check refuses an active connection whose refresh token has no
    // refresh task.
    assert!(
        db.activate(&account, &tokens(Some(3600)), None)
            .await
            .is_err()
    );
    let account = db.connected_account(&id).await.unwrap();
    assert_eq!(account.status, ConnectionStatus::Pending);
    assert!(account.tokens.is_none());
}

#[sqlx::test(migrator = "crate::db::MIGRATOR")]
async fn counts_a_token_without_a_refresh_token_as_complete_without_a_task(pool: PgPool) {
    let db = db(pool);
    let config = github_config(&db).await;
    let id = pending(&db, &config, "org_1", "state-1").await;
    let account = db.connected_account(&id).await.unwrap();

    db.activate(&account, &no_expiry("access-1"), None)
        .await
        .unwrap();
    let active = db.connected_account(&id).await.unwrap();
    assert!(active.is_complete());
    assert!(active.refresh_task.is_none());
}

#[sqlx::test(migrator = "crate::db::MIGRATOR")]
async fn disables_a_consent_without_keeping_tokens(pool: PgPool) {
    let db = db(pool);
    let config = github_config(&db).await;
    let id = pending(&db, &config, "org_1", "state-1").await;

    db.disable_consent(&id, "no refresh task").await.unwrap();
    let account = db.connected_account(&id).await.unwrap();
    assert_eq!(account.status, ConnectionStatus::Disabled);
    assert!(account.tokens.is_none());
    assert!(!account.is_complete());
}

#[sqlx::test(migrator = "crate::db::MIGRATOR")]
async fn moves_to_the_next_generation_with_the_next_task_and_keeps_the_refresh_token(pool: PgPool) {
    let db = db(pool);
    let id = active(&db, "org_1", "state-1", Some(60)).await;

    let refreshed = db
        .lock_for_refresh(&id)
        .await
        .unwrap()
        .refreshed(&no_expiry("access-2"), Some(&refresh_task("refresh-2")))
        .await
        .unwrap();
    assert_eq!(refreshed.token_generation, 2);
    assert_eq!(refreshed.refresh_task.as_deref(), Some("refresh-2"));
    let held = refreshed.tokens.unwrap();
    assert_eq!(held.access_token.expose_secret(), "access-2");
    assert_eq!(held.refresh_token.unwrap().expose_secret(), "refresh-1");
}

#[sqlx::test(migrator = "crate::db::MIGRATOR")]
async fn counts_failed_refreshes_and_starts_over_after_one_works(pool: PgPool) {
    let db = db(pool);
    let id = active(&db, "org_1", "state-1", Some(60)).await;
    for _ in 0..2 {
        db.lock_for_refresh(&id)
            .await
            .unwrap()
            .failed("503")
            .await
            .unwrap();
    }
    let account = db.connected_account(&id).await.unwrap();
    assert_eq!(account.refresh_failures, 2);
    assert_eq!(account.token_generation, 1, "a failure keeps the tokens");
    assert_eq!(account.refresh_task.as_deref(), Some("refresh-1"));
    db.lock_for_refresh(&id)
        .await
        .unwrap()
        .refreshed(&tokens(Some(3600)), Some(&refresh_task("refresh-2")))
        .await
        .unwrap();
    assert_eq!(db.connected_account(&id).await.unwrap().refresh_failures, 0);
}

#[sqlx::test(migrator = "crate::db::MIGRATOR")]
async fn revokes_with_its_event_and_clears_the_refresh_task(pool: PgPool) {
    let db = db(pool);
    let id = active(&db, "org_1", "state-1", Some(3600)).await;

    let (event, _) = db
        .lock_for_refresh(&id)
        .await
        .unwrap()
        .revoke("401 bad credentials")
        .await
        .unwrap();
    assert_eq!(event.event_type, EventType::ConnectionRevoked);
    assert_eq!(event.connected_account_id, id);
    let account = db.connected_account(&id).await.unwrap();
    assert_eq!(account.status, ConnectionStatus::Expired);
    assert!(account.refresh_task.is_none());
}

#[sqlx::test(migrator = "crate::db::MIGRATOR")]
async fn makes_a_second_refresh_wait_for_the_first(pool: PgPool) {
    let db = db(pool.clone());
    let id = active(&db, "org_1", "state-1", Some(60)).await;

    let first = db.lock_for_refresh(&id).await.unwrap();
    let second = {
        let db = db.clone();
        let id = id.clone();
        tokio::spawn(async move { db.lock_for_refresh(&id).await.unwrap() })
    };
    // The second refresh is waiting on the first one's row lock, as Postgres reports it.
    tokio::time::timeout(std::time::Duration::from_secs(10), async {
        loop {
            let waiting = sqlx::query_scalar!(
                r#"SELECT count(*) AS "waiting!" FROM pg_stat_activity
                   WHERE wait_event_type = 'Lock' AND datname = current_database()"#
            )
            .fetch_one(&pool)
            .await
            .unwrap();
            if waiting > 0 {
                break;
            }
            tokio::task::yield_now().await;
        }
    })
    .await
    .expect("the second refresh waits for the lock");
    assert!(!second.is_finished());
    first
        .refreshed(&no_expiry("access-2"), Some(&refresh_task("refresh-2")))
        .await
        .unwrap();
    let second = second.await.unwrap();
    assert_eq!(second.account.token_generation, 2);
    second.release().await.unwrap();
}

#[sqlx::test(migrator = "crate::db::MIGRATOR")]
async fn replaces_the_refresh_task_only_on_the_same_generation(pool: PgPool) {
    let db = db(pool);
    let id = active(&db, "org_1", "state-1", Some(3600)).await;

    assert!(
        db.set_refresh_task(&id, 1, &refresh_task("refresh-now"))
            .await
            .unwrap()
    );
    assert_eq!(
        db.connected_account(&id)
            .await
            .unwrap()
            .refresh_task
            .as_deref(),
        Some("refresh-now")
    );
    assert!(
        !db.set_refresh_task(&id, 7, &refresh_task("other"))
            .await
            .unwrap()
    );
}

#[sqlx::test(migrator = "crate::db::MIGRATOR")]
async fn answers_the_pending_refresh_task_when_deleting(pool: PgPool) {
    let db = db(pool);
    let id = active(&db, "org_1", "state-1", Some(3600)).await;
    assert_eq!(
        db.delete_connected_account(&id).await.unwrap().as_deref(),
        Some("refresh-1")
    );
    assert!(db.connected_account(&id).await.is_err());
    assert!(db.delete_connected_account(&id).await.is_err());
}

#[sqlx::test(migrator = "crate::db::MIGRATOR")]
async fn fails_a_consent_only_while_it_is_open(pool: PgPool) {
    let db = db(pool);
    let config = github_config(&db).await;
    let open = pending(&db, &config, "org_1", "s1").await;
    assert!(db.fail_open_consent(&open).await.unwrap());
    assert_eq!(
        db.connected_account(&open).await.unwrap().status,
        ConnectionStatus::Failed
    );
    let finished = active(&db, "org_2", "s2", Some(3600)).await;
    assert!(!db.fail_open_consent(&finished).await.unwrap());
}

#[sqlx::test(migrator = "crate::db::MIGRATOR")]
async fn lists_connections_by_attempt_a_page_at_a_time(pool: PgPool) {
    let db = db(pool);
    let config = github_config(&db).await;
    for state in ["s1", "s2", "s3"] {
        pending(&db, &config, "org_1", state).await;
    }
    let filter = AccountFilter {
        alias: Some("attempt-1".into()),
        provider: Some("github".into()),
        ..Default::default()
    };
    let first = db
        .list_connected_accounts(
            &filter,
            &Cursor {
                limit: 2,
                ..Default::default()
            },
        )
        .await
        .unwrap();
    assert_eq!(first.items.len(), 2);
    let second = db
        .list_connected_accounts(
            &filter,
            &Cursor {
                after: first.next.clone(),
                limit: 2,
                ..Default::default()
            },
        )
        .await
        .unwrap();
    assert_eq!(second.items.len(), 1);
    assert!(second.next.is_none());
    let back = db
        .list_connected_accounts(
            &filter,
            &Cursor {
                before: second.previous.clone(),
                limit: 2,
                ..Default::default()
            },
        )
        .await
        .unwrap();
    assert_eq!(
        back.items.iter().map(|a| &a.id).collect::<Vec<_>>(),
        first.items.iter().map(|a| &a.id).collect::<Vec<_>>()
    );
}

#[sqlx::test(migrator = "crate::db::MIGRATOR")]
async fn keeps_unscheduled_new_tokens_disabled_until_restored(pool: PgPool) {
    let db = db(pool);
    let id = active(&db, "org_1", "state-1", Some(60)).await;

    let disabled = db
        .lock_for_refresh(&id)
        .await
        .unwrap()
        .refreshed_unscheduled(&tokens(Some(3600)), "no scheduler")
        .await
        .unwrap();
    assert_eq!(disabled.status, ConnectionStatus::Disabled);
    assert_eq!(disabled.token_generation, 2, "the new tokens are kept");
    assert!(disabled.refresh_task.is_none());
    assert!(!disabled.is_complete());

    assert!(
        !db.restore(&id, 1, &refresh_task("old")).await.unwrap(),
        "wrong generation"
    );
    assert!(
        db.restore(&id, 2, &refresh_task("refresh-2"))
            .await
            .unwrap()
    );
    let restored = db.connected_account(&id).await.unwrap();
    assert_eq!(restored.status, ConnectionStatus::Active);
    assert_eq!(restored.refresh_task.as_deref(), Some("refresh-2"));
    assert!(restored.is_complete());
    assert!(
        !db.restore(&id, 2, &refresh_task("again")).await.unwrap(),
        "only once"
    );
}

#[sqlx::test(migrator = "crate::db::MIGRATOR")]
async fn lists_connections_of_several_tenants_each_decrypted_with_its_own_key(pool: PgPool) {
    let writer = db(pool.clone());
    for (user, state) in [("org_1", "s1"), ("org_2", "s2"), ("org_3", "s3")] {
        active(&writer, user, state, Some(3600)).await;
    }
    // A fresh db has no key in memory: the page reads them in one go.
    let reader = db(pool);
    let page = reader
        .list_connected_accounts(
            &AccountFilter::default(),
            &Cursor {
                limit: 10,
                ..Default::default()
            },
        )
        .await
        .unwrap();
    assert_eq!(page.items.len(), 3);
    for account in page.items {
        assert_eq!(
            account.tokens.unwrap().access_token.expose_secret(),
            "access-1"
        );
    }
}

#[sqlx::test(migrator = "crate::db::MIGRATOR")]
async fn refuses_a_cursor_naming_a_deleted_connection(pool: PgPool) {
    let db = db(pool);

    let failure = db
        .list_connected_accounts(
            &AccountFilter::default(),
            &Cursor {
                after: Some("ca_gone".into()),
                limit: 2,
                ..Default::default()
            },
        )
        .await
        .unwrap_err();

    assert!(matches!(
        failure.current_context(),
        osiris_domain::DomainError::Invalid(_)
    ));
}
