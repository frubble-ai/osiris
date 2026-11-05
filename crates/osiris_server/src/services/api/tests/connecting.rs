//! Connecting: the consent and its expiry task, the callback (accepted once), the tokens saved
//! with their first refresh task, the event, and the completeness rule.

use crate::testing::generate_connect_attempt::generate_connect_attempt;
use crate::testing::{API_KEY, setup_test_context};
use axum::http::StatusCode;
use osiris_domain::EventType;
use osiris_providers::{
    github::mocks::MockGithubNetwork,
    mocks::generate_token_answer::{generate_token_answer, generate_token_answer_without_refresh},
};
use osiris_tasks::{RefreshReason, TaskPayload, names};
use sqlx::PgPool;
use time::{Duration, OffsetDateTime};
use url::Url;

#[sqlx::test(migrator = "crate::db::MIGRATOR")]
async fn connects_through_the_consent_and_sends_the_browser_back_once(pool: PgPool) {
    let app = setup_test_context(pool).await;
    MockGithubNetwork::on(&app.provider)
        .answer_code_exchange(generate_token_answer(Some(28800)))
        .await;
    let config = app.github_config().await;
    let attempt = generate_connect_attempt(&config);
    let (id, state) = app.start_connect_with(&attempt).await;
    let callback_url = Url::parse(attempt["callback_url"].as_str().unwrap()).unwrap();

    let callback = app
        .send(
            "GET",
            &format!("/oauth/callback?state={state}&code=the-code"),
            None,
            None,
            &[],
        )
        .await;
    assert_eq!(callback.status, StatusCode::SEE_OTHER);
    let back = Url::parse(callback.headers["location"].to_str().unwrap()).unwrap();
    assert_eq!(back.host_str(), callback_url.host_str());
    let query: Vec<(String, String)> = back
        .query_pairs()
        .map(|(k, v)| (k.into_owned(), v.into_owned()))
        .collect();
    let (_, connection) = callback_url
        .query_pairs()
        .find(|(name, _)| name == "connection")
        .unwrap();
    assert!(query.contains(&("connection".into(), connection.into_owned())));
    assert!(query.contains(&("connected_account_id".into(), id.clone())));
    assert!(query.contains(&("status".into(), "active".into())));

    let account = app.account(&id).await;
    assert_eq!(account["status"], "active");
    assert_eq!(account["setup_complete"], true);
    assert!(
        account.get("access_token").is_none(),
        "tokens never leave osiris"
    );
    // The first refresh task is saved with the tokens, and is pending at the scheduler.
    let task = names::refresh(&id, 1);
    assert_eq!(account["refresh_task"], task.as_str());
    let pending = app
        .tasks
        .get(&task)
        .expect("the first refresh is scheduled");
    let TaskPayload::ConnectionRefresh(refresh) = &pending.payload else {
        panic!("a refresh task");
    };
    assert_eq!(refresh.generation, 1);
    assert_eq!(refresh.tenant_id, attempt["user_id"].as_str().unwrap());
    assert_eq!(refresh.reason, RefreshReason::Expiry);
    // Due a margin (five minutes) before the token's eight hours run out.
    let lead = OffsetDateTime::now_utc() + Duration::seconds(28800) - pending.run_at;
    assert!(
        lead >= Duration::minutes(4) && lead <= Duration::minutes(6),
        "{lead}"
    );
    // The consent is finished, so its expiry task was cancelled.
    assert!(app.tasks.cancelled().contains(&names::consent(&id)));

    let events = app.events();
    assert_eq!(events.len(), 1);
    assert_eq!(events[0].event_type, EventType::ConnectionActive);

    let again = app
        .send(
            "GET",
            &format!("/oauth/callback?state={state}&code=the-code"),
            None,
            None,
            &[],
        )
        .await;
    assert_eq!(
        again.status,
        StatusCode::BAD_REQUEST,
        "a consent's state works once"
    );
}

#[sqlx::test(migrator = "crate::db::MIGRATOR")]
async fn fails_a_refused_consent_and_says_so_on_the_way_back(pool: PgPool) {
    let app = setup_test_context(pool).await;
    let config = app.github_config().await;
    let (id, state) = app.start_connect(&config).await;

    let callback = app
        .send(
            "GET",
            &format!("/oauth/callback?state={state}&error=access_denied"),
            None,
            None,
            &[],
        )
        .await;
    assert!(
        callback.headers["location"]
            .to_str()
            .unwrap()
            .contains("status=failed")
    );
    let account = app
        .send(
            "GET",
            &format!("/v1/connected-accounts/{id}"),
            Some(API_KEY),
            None,
            &[],
        )
        .await;
    assert_eq!(account.body["status"], "failed");
}

#[sqlx::test(migrator = "crate::db::MIGRATOR")]
async fn replays_a_connect_retried_with_the_same_idempotency_key(pool: PgPool) {
    let app = setup_test_context(pool).await;
    let config = app.github_config().await;
    let body = generate_connect_attempt(&config);
    let first = app
        .send(
            "POST",
            "/v1/connected-accounts",
            Some(API_KEY),
            Some(body.clone()),
            &[("idempotency-key", "k1")],
        )
        .await;
    let retry = app
        .send(
            "POST",
            "/v1/connected-accounts",
            Some(API_KEY),
            Some(body),
            &[("idempotency-key", "k1")],
        )
        .await;
    assert_eq!(retry.status, StatusCode::CREATED);
    assert_eq!(retry.body["id"], first.body["id"]);
    assert_eq!(retry.headers["idempotent-replayed"], "true");
    let listed = app
        .send("GET", "/v1/connected-accounts", Some(API_KEY), None, &[])
        .await;
    assert_eq!(listed.body["data"].as_array().unwrap().len(), 1);
}

#[sqlx::test(migrator = "crate::db::MIGRATOR")]
async fn schedules_the_consent_s_expiry_when_it_opens(pool: PgPool) {
    let app = setup_test_context(pool).await;
    let config = app.github_config().await;
    let (id, _) = app.start_connect(&config).await;
    let task = app
        .tasks
        .get(&names::consent(&id))
        .expect("the consent's expiry is scheduled");
    let ttl = task.run_at - OffsetDateTime::now_utc();
    assert!(
        ttl > Duration::minutes(14) && ttl <= Duration::minutes(15),
        "{ttl}"
    );
}

#[sqlx::test(migrator = "crate::db::MIGRATOR")]
async fn refuses_to_open_a_consent_whose_expiry_cannot_be_scheduled(pool: PgPool) {
    let app = setup_test_context(pool).await;
    let config = app.github_config().await;
    app.tasks.fail(true);
    let answer = app
        .send(
            "POST",
            "/v1/connected-accounts",
            Some(API_KEY),
            Some(generate_connect_attempt(&config)),
            &[],
        )
        .await;
    assert_eq!(answer.status, StatusCode::INTERNAL_SERVER_ERROR);
    let listed = app
        .send("GET", "/v1/connected-accounts", Some(API_KEY), None, &[])
        .await;
    assert_eq!(listed.body["data"][0]["status"], "failed");
}

#[sqlx::test(migrator = "crate::db::MIGRATOR")]
async fn keeps_no_tokens_and_disables_the_connection_when_the_first_refresh_cannot_be_scheduled(
    pool: PgPool,
) {
    let app = setup_test_context(pool).await;
    MockGithubNetwork::on(&app.provider)
        .answer_code_exchange(generate_token_answer(Some(28800)))
        .await;
    let config = app.github_config().await;
    let (id, state) = app.start_connect(&config).await;
    app.tasks.fail(true);

    let callback = app
        .send(
            "GET",
            &format!("/oauth/callback?state={state}&code=c"),
            None,
            None,
            &[],
        )
        .await;
    assert!(
        callback.headers["location"]
            .to_str()
            .unwrap()
            .contains("status=disabled")
    );
    let account = app.account(&id).await;
    assert_eq!(account["status"], "disabled");
    assert_eq!(account["setup_complete"], false);
    assert!(account["refresh_task"].is_null());
    let stored = app
        .use_cases
        .context()
        .db
        .connected_account(&id)
        .await
        .unwrap();
    assert!(
        stored.tokens.is_none(),
        "no token is kept without its refresh"
    );
    assert!(
        app.events().is_empty(),
        "an incomplete connection is not announced"
    );

    // Actions refuse it: the user must connect again.
    let action = app
        .send(
            "POST",
            &format!("/v1/connected-accounts/{id}/actions/github.list_installations"),
            Some(API_KEY),
            None,
            &[],
        )
        .await;
    assert_eq!(action.status, StatusCode::CONFLICT);
}

#[sqlx::test(migrator = "crate::db::MIGRATOR")]
async fn reports_an_active_row_missing_its_refresh_task_as_disabled(pool: PgPool) {
    let app = setup_test_context(pool.clone()).await;
    let id = app.connected(28800).await;
    // The database's completeness check prevents this row. The check is dropped here to confirm
    // that the API would still report such a row as disabled, never as active.
    sqlx::query!("ALTER TABLE connected_account DROP CONSTRAINT connected_account_complete")
        .execute(&pool)
        .await
        .unwrap();
    sqlx::query!(
        "UPDATE connected_account SET refresh_task = NULL WHERE id = $1",
        id
    )
    .execute(&pool)
    .await
    .unwrap();
    let account = app.account(&id).await;
    assert_eq!(account["status"], "disabled");
    assert_eq!(account["setup_complete"], false);
}

#[sqlx::test(migrator = "crate::db::MIGRATOR")]
async fn disables_a_connection_whose_provider_should_have_sent_a_refresh_token(pool: PgPool) {
    let app = setup_test_context(pool).await;
    // GitHub issues refresh tokens; an app with expiring tokens turned off sends none.
    MockGithubNetwork::on(&app.provider)
        .answer_code_exchange(generate_token_answer_without_refresh(None))
        .await;
    let config = app.github_config().await;
    let (id, state) = app.start_connect(&config).await;

    let callback = app
        .send(
            "GET",
            &format!("/oauth/callback?state={state}&code=c"),
            None,
            None,
            &[],
        )
        .await;
    assert!(
        callback.headers["location"]
            .to_str()
            .unwrap()
            .contains("status=disabled")
    );
    let account = app.account(&id).await;
    assert_eq!(account["status"], "disabled");
    assert_eq!(account["setup_complete"], false);
    assert!(
        account["last_error"]
            .as_str()
            .unwrap()
            .contains("refresh token")
    );
    assert!(
        app.use_cases
            .context()
            .db
            .connected_account(&id)
            .await
            .unwrap()
            .tokens
            .is_none()
    );
    assert!(app.tasks.get(&names::refresh(&id, 1)).is_none());
}

#[sqlx::test(migrator = "crate::db::MIGRATOR")]
async fn stores_a_fixed_reason_when_the_consent_comes_back_with_an_unknown_error(pool: PgPool) {
    let app = setup_test_context(pool).await;
    let config = app.github_config().await;
    let (id, state) = app.start_connect(&config).await;
    app.send(
        "GET",
        &format!("/oauth/callback?state={state}&error=%3Cscript%3Ealert(1)%3C%2Fscript%3E"),
        None,
        None,
        &[],
    )
    .await;
    let account = app.account(&id).await;
    assert_eq!(account["status"], "failed");
    assert_eq!(account["last_error"], "the provider refused the consent");
}
