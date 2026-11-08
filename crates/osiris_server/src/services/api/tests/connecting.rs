//! Connecting: the consent and its expiry task, the callback (accepted once), the tokens saved
//! with their first refresh task, the event, and the completeness rule.

use crate::testing::generate_connect_attempt::generate_connect_attempt;
use crate::testing::{API_KEY, setup_test_context, setup_test_context_with};
use axum::http::StatusCode;
use osiris_domain::EventType;
use osiris_providers::mocks::{
    MockFakeNetwork,
    generate_token_answer::{generate_token_answer, generate_token_answer_without_refresh},
};
use osiris_tasks::{RefreshReason, TaskPayload, names};
use sqlx::PgPool;
use time::{Duration, OffsetDateTime};
use url::Url;

#[sqlx::test(migrator = "crate::db::MIGRATOR")]
async fn connects_through_the_consent_and_sends_the_browser_back_once(pool: PgPool) {
    let app = setup_test_context(pool).await;
    MockFakeNetwork::on(&app.provider)
        .answer_code_exchange(generate_token_answer(Some(28800)))
        .await;
    let config = app.fake_config().await;
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
        .map(|(name, value)| (name.into_owned(), value.into_owned()))
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
    let config = app.fake_config().await;
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
    let config = app.fake_config().await;
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
    let config = app.fake_config().await;
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
    let config = app.fake_config().await;
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
    MockFakeNetwork::on(&app.provider)
        .answer_code_exchange(generate_token_answer(Some(28800)))
        .await;
    let config = app.fake_config().await;
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
    // The fake provider issues refresh tokens, so an answer without one leaves nothing to refresh.
    MockFakeNetwork::on(&app.provider)
        .answer_code_exchange(generate_token_answer_without_refresh(None))
        .await;
    let config = app.fake_config().await;
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
    let config = app.fake_config().await;
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

async fn finishes_with(app: &crate::testing::TestContext, query: &str) -> axum::http::HeaderValue {
    app.send("GET", &format!("/oauth/callback?{query}"), None, None, &[])
        .await
        .headers
        .get("location")
        .cloned()
        .unwrap_or_else(|| axum::http::HeaderValue::from_static(""))
}

async fn assert_consent_failed(app: &crate::testing::TestContext, id: &str, reason: &str) {
    let account = app.account(id).await;
    assert_eq!(account["status"], "failed");
    assert_eq!(account["last_error"], reason);
}

#[sqlx::test(migrator = "crate::db::MIGRATOR")]
async fn refuses_a_callback_without_a_state_or_with_one_never_issued(pool: PgPool) {
    let app = setup_test_context(pool).await;

    let missing = app
        .send("GET", "/oauth/callback?code=c", None, None, &[])
        .await;
    let unknown = app
        .send(
            "GET",
            "/oauth/callback?state=never-issued&code=c",
            None,
            None,
            &[],
        )
        .await;

    assert_eq!(missing.status, StatusCode::BAD_REQUEST);
    assert_eq!(unknown.status, StatusCode::BAD_REQUEST);
}

#[sqlx::test(migrator = "crate::db::MIGRATOR")]
async fn fails_a_callback_that_carries_no_code(pool: PgPool) {
    let app = setup_test_context(pool).await;
    let config = app.fake_config().await;
    let (id, state) = app.start_connect(&config).await;

    let back = finishes_with(&app, &format!("state={state}")).await;

    assert!(back.to_str().unwrap().contains("status=failed"));
    assert_consent_failed(&app, &id, "the provider sent no code").await;
}

#[sqlx::test(migrator = "crate::db::MIGRATOR")]
async fn fails_a_callback_that_came_after_the_consent_expired(pool: PgPool) {
    let app = setup_test_context_with(pool, |config| {
        config.settings.consent.ttl_seconds = 0;
    })
    .await;
    let config = app.fake_config().await;
    let (id, state) = app.start_connect(&config).await;

    let back = finishes_with(&app, &format!("state={state}&code=c")).await;

    assert!(back.to_str().unwrap().contains("status=failed"));
    assert_consent_failed(&app, &id, "the consent took too long").await;
}

#[sqlx::test(migrator = "crate::db::MIGRATOR")]
async fn fails_a_consent_whose_code_the_provider_refuses(pool: PgPool) {
    let app = setup_test_context(pool).await;
    MockFakeNetwork::on(&app.provider)
        .answer_code_exchange_status(400, serde_json::json!({ "error": "bad_verification_code" }))
        .await;
    let config = app.fake_config().await;
    let (id, state) = app.start_connect(&config).await;

    let back = finishes_with(&app, &format!("state={state}&code=c")).await;

    assert!(back.to_str().unwrap().contains("status=failed"));
    assert_consent_failed(&app, &id, "the provider refused the consent's code").await;
}

#[sqlx::test(migrator = "crate::db::MIGRATOR")]
async fn fails_a_consent_when_the_provider_cannot_be_reached(pool: PgPool) {
    let app = setup_test_context(pool).await;
    MockFakeNetwork::on(&app.provider)
        .answer_code_exchange_status(503, serde_json::json!({}))
        .await;
    let config = app.fake_config().await;
    let (id, state) = app.start_connect(&config).await;

    let back = finishes_with(&app, &format!("state={state}&code=c")).await;

    assert!(back.to_str().unwrap().contains("status=failed"));
    assert_consent_failed(&app, &id, "the provider could not be reached").await;
}
