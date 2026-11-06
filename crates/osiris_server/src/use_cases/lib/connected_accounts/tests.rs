//! `connected_account`: the consent, tokens saved with their refresh task, the token generation,
//! refresh locking, and the completeness rule.

use crate::testing::db_fixtures::{
    active, active_with, db, generate_secret, generate_state, generate_tokens, generate_user_id,
    github_config, no_expiry, pending, pending_with, refresh_task,
};
use osiris_common::secret::ExposeSecret;
use osiris_domain::{AccountFilter, Cursor};
use osiris_domain::{ConnectionStatus, EventType};
use sqlx::PgPool;

#[sqlx::test(migrator = "crate::db::MIGRATOR")]
async fn lets_a_consent_be_claimed_once(pool: PgPool) {
    let db = db(pool);
    let config = github_config(&db).await;
    let user = generate_user_id();
    let state = generate_state();
    let verifier = generate_secret("verifier");
    let id = pending_with(&db, &config, &user, &state, &verifier).await;

    let claimed = db.claim_consent(&state).await.unwrap().unwrap();
    assert_eq!(claimed.account.id, id);
    assert_eq!(
        claimed
            .pkce_verifier
            .as_ref()
            .map(ExposeSecret::expose_secret),
        Some(verifier.as_str())
    );
    assert!(!claimed.expired);
    assert!(db.claim_consent(&state).await.unwrap().is_none());
}

#[sqlx::test(migrator = "crate::db::MIGRATOR")]
async fn saves_the_tokens_with_their_refresh_task_as_generation_one(pool: PgPool) {
    let db = db(pool);
    let tokens = generate_tokens(Some(3600));
    let id = active_with(&db, &generate_user_id(), &generate_state(), &tokens).await;

    let account = db.connected_account(&id).await.unwrap();
    assert_eq!(account.status, ConnectionStatus::Active);
    assert_eq!(account.token_generation, 1);
    assert!(account.refresh_task.is_some());
    assert!(account.refresh_task_at.is_some());
    assert!(account.is_complete());
    assert_eq!(
        account.tokens.unwrap().access_token.expose_secret(),
        tokens.access_token.expose_secret()
    );
}

#[sqlx::test(migrator = "crate::db::MIGRATOR")]
async fn refuses_to_save_a_refreshable_token_without_its_refresh_task(pool: PgPool) {
    let db = db(pool);
    let config = github_config(&db).await;
    let id = pending(&db, &config, &generate_user_id(), &generate_state()).await;
    let account = db.connected_account(&id).await.unwrap();

    // The database's completeness check refuses an active connection whose refresh token has no
    // refresh task.
    let refused = db
        .activate(&account, &generate_tokens(Some(3600)), None)
        .await
        .unwrap_err();
    assert!(matches!(
        refused.current_context(),
        osiris_domain::DomainError::Internal
    ));
    let detail = format!("{refused:?}");
    assert!(detail.contains("connected_account_complete"), "{detail}");
    let account = db.connected_account(&id).await.unwrap();
    assert_eq!(account.status, ConnectionStatus::Pending);
    assert!(account.tokens.is_none());
}

#[sqlx::test(migrator = "crate::db::MIGRATOR")]
async fn counts_a_token_without_a_refresh_token_as_complete_without_a_task(pool: PgPool) {
    let db = db(pool);
    let config = github_config(&db).await;
    let id = pending(&db, &config, &generate_user_id(), &generate_state()).await;
    let account = db.connected_account(&id).await.unwrap();

    db.activate(&account, &no_expiry(&generate_secret("access")), None)
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
    let id = pending(&db, &config, &generate_user_id(), &generate_state()).await;

    db.disable_consent(&id, "no refresh task").await.unwrap();
    let account = db.connected_account(&id).await.unwrap();
    assert_eq!(account.status, ConnectionStatus::Disabled);
    assert!(account.tokens.is_none());
    assert!(!account.is_complete());
}

#[sqlx::test(migrator = "crate::db::MIGRATOR")]
async fn moves_to_the_next_generation_with_the_next_task_and_keeps_the_refresh_token(pool: PgPool) {
    let db = db(pool);
    let original = generate_tokens(Some(60));
    let id = active_with(&db, &generate_user_id(), &generate_state(), &original).await;
    let next_access = generate_secret("access");
    let next_task = generate_secret("refresh");

    let refreshed = db
        .lock_for_refresh(&id)
        .await
        .unwrap()
        .refreshed(&no_expiry(&next_access), Some(&refresh_task(&next_task)))
        .await
        .unwrap();
    assert_eq!(refreshed.token_generation, 2);
    assert_eq!(refreshed.refresh_task.as_deref(), Some(next_task.as_str()));
    let held = refreshed.tokens.unwrap();
    assert_eq!(held.access_token.expose_secret(), next_access);
    assert_eq!(
        held.refresh_token.unwrap().expose_secret(),
        original.refresh_token.unwrap().expose_secret()
    );
}

#[sqlx::test(migrator = "crate::db::MIGRATOR")]
async fn counts_failed_refreshes_and_starts_over_after_one_works(pool: PgPool) {
    let db = db(pool);
    let id = active(&db, &generate_user_id(), &generate_state(), Some(60)).await;
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
    assert!(account.refresh_task.is_some());
    let next_task = generate_secret("refresh");
    db.lock_for_refresh(&id)
        .await
        .unwrap()
        .refreshed(
            &generate_tokens(Some(3600)),
            Some(&refresh_task(&next_task)),
        )
        .await
        .unwrap();
    assert_eq!(db.connected_account(&id).await.unwrap().refresh_failures, 0);
}

#[sqlx::test(migrator = "crate::db::MIGRATOR")]
async fn revokes_with_its_event_and_clears_the_refresh_task(pool: PgPool) {
    let db = db(pool);
    let id = active(&db, &generate_user_id(), &generate_state(), Some(3600)).await;

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
    let db = db(pool);
    let user = generate_secret("org");
    let state = generate_secret("state");
    let id = active(&db, &user, &state, Some(60)).await;
    let next_access = generate_secret("access");
    let next_task = generate_secret("refresh");

    let first = db.lock_for_refresh(&id).await.unwrap();
    let mut second = {
        let db = db.clone();
        let id = id.clone();
        tokio::spawn(async move { db.lock_for_refresh(&id).await.unwrap() })
    };
    // The second refresh waits on the in-process flight before taking a pool connection.
    tokio::select! {
        biased;
        _ = &mut second => panic!("the second refresh must wait for the first"),
        () = tokio::time::sleep(std::time::Duration::from_millis(100)) => {}
    }
    assert!(!second.is_finished());
    first
        .refreshed(&no_expiry(&next_access), Some(&refresh_task(&next_task)))
        .await
        .unwrap();
    let second = second.await.unwrap();
    assert_eq!(second.account.token_generation, 2);
    second.release().await.unwrap();
}

#[sqlx::test(migrator = "crate::db::MIGRATOR")]
async fn replaces_the_refresh_task_only_on_the_same_generation(pool: PgPool) {
    let db = db(pool);
    let id = active(&db, &generate_user_id(), &generate_state(), Some(3600)).await;

    let next_task = generate_secret("refresh");
    assert!(
        db.set_refresh_task(&id, 1, &refresh_task(&next_task))
            .await
            .unwrap()
    );
    assert_eq!(
        db.connected_account(&id)
            .await
            .unwrap()
            .refresh_task
            .as_deref(),
        Some(next_task.as_str())
    );
    assert!(
        !db.set_refresh_task(&id, 7, &refresh_task(&generate_secret("refresh")))
            .await
            .unwrap()
    );
}

#[sqlx::test(migrator = "crate::db::MIGRATOR")]
async fn answers_the_pending_refresh_task_when_deleting(pool: PgPool) {
    let db = db(pool);
    let id = active(&db, &generate_user_id(), &generate_state(), Some(3600)).await;
    let task = db
        .connected_account(&id)
        .await
        .unwrap()
        .refresh_task
        .clone();
    assert_eq!(
        db.delete_connected_account(&id).await.unwrap().as_deref(),
        task.as_deref()
    );
    assert!(db.connected_account(&id).await.is_err());
    assert!(db.delete_connected_account(&id).await.is_err());
}

#[sqlx::test(migrator = "crate::db::MIGRATOR")]
async fn fails_a_consent_only_while_it_is_open(pool: PgPool) {
    let db = db(pool);
    let config = github_config(&db).await;
    let open = pending(&db, &config, &generate_user_id(), &generate_state()).await;
    assert!(db.fail_open_consent(&open).await.unwrap());
    assert_eq!(
        db.connected_account(&open).await.unwrap().status,
        ConnectionStatus::Failed
    );
    let finished = active(&db, &generate_user_id(), &generate_state(), Some(3600)).await;
    assert!(!db.fail_open_consent(&finished).await.unwrap());
}

#[sqlx::test(migrator = "crate::db::MIGRATOR")]
async fn lists_connections_by_attempt_a_page_at_a_time(pool: PgPool) {
    let db = db(pool);
    let config = github_config(&db).await;
    let user = generate_user_id();
    for _ in 0..3 {
        pending(&db, &config, &user, &generate_state()).await;
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
        back.items
            .iter()
            .map(|account| &account.id)
            .collect::<Vec<_>>(),
        first
            .items
            .iter()
            .map(|account| &account.id)
            .collect::<Vec<_>>()
    );
}

#[sqlx::test(migrator = "crate::db::MIGRATOR")]
async fn keeps_unscheduled_new_tokens_disabled_until_restored(pool: PgPool) {
    let db = db(pool);
    let id = active(&db, &generate_user_id(), &generate_state(), Some(60)).await;

    let disabled = db
        .lock_for_refresh(&id)
        .await
        .unwrap()
        .refreshed_unscheduled(&generate_tokens(Some(3600)), "no scheduler")
        .await
        .unwrap();
    assert_eq!(disabled.status, ConnectionStatus::Disabled);
    assert_eq!(disabled.token_generation, 2, "the new tokens are kept");
    assert!(disabled.refresh_task.is_none());
    assert!(!disabled.is_complete());

    assert!(
        !db.restore(&id, 1, &refresh_task(&generate_secret("refresh")))
            .await
            .unwrap(),
        "wrong generation"
    );
    let restored_task = generate_secret("refresh");
    assert!(
        db.restore(&id, 2, &refresh_task(&restored_task))
            .await
            .unwrap()
    );
    let restored = db.connected_account(&id).await.unwrap();
    assert_eq!(restored.status, ConnectionStatus::Active);
    assert_eq!(
        restored.refresh_task.as_deref(),
        Some(restored_task.as_str())
    );
    assert!(restored.is_complete());
    assert!(
        !db.restore(&id, 2, &refresh_task(&generate_secret("refresh")))
            .await
            .unwrap(),
        "only once"
    );
}

#[sqlx::test(migrator = "crate::db::MIGRATOR")]
async fn lists_connections_of_several_tenants_each_decrypted_with_its_own_key(pool: PgPool) {
    let writer = db(pool.clone());
    let tokens = generate_tokens(Some(3600));
    for _ in 0..3 {
        active_with(&writer, &generate_user_id(), &generate_state(), &tokens).await;
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
            tokens.access_token.expose_secret()
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
