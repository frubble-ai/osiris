//! Idempotency keys only for an authenticated caller: nothing unauthenticated is stored or replayed.

use crate::testing::generate_connect_attempt::generate_connect_attempt;
use crate::testing::{ADMIN_KEY, API_KEY, setup_test_context};
use axum::http::StatusCode;
use sqlx::PgPool;

#[sqlx::test(migrator = "crate::db::MIGRATOR")]
async fn never_replays_a_stored_answer_to_a_caller_without_a_key(pool: PgPool) {
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
    assert_eq!(first.status, StatusCode::CREATED);

    let stranger = app
        .send(
            "POST",
            "/v1/connected-accounts",
            None,
            Some(body.clone()),
            &[("idempotency-key", "k1")],
        )
        .await;
    assert_eq!(stranger.status, StatusCode::UNAUTHORIZED);
    assert!(
        stranger.body.get("redirect_url").is_none(),
        "no stored answer leaks"
    );

    let other_key = app
        .send(
            "POST",
            "/v1/connected-accounts",
            Some(ADMIN_KEY),
            Some(body),
            &[("idempotency-key", "k1")],
        )
        .await;
    assert_eq!(
        other_key.status,
        StatusCode::UNAUTHORIZED,
        "the admin key's scope holds no answer, and the route refuses it"
    );
}

#[sqlx::test(migrator = "crate::db::MIGRATOR")]
async fn lets_an_unauthenticated_attempt_leave_the_key_free(pool: PgPool) {
    let app = setup_test_context(pool).await;
    let config = app.github_config().await;
    let body = generate_connect_attempt(&config);

    let refused = app
        .send(
            "POST",
            "/v1/connected-accounts",
            None,
            Some(body.clone()),
            &[("idempotency-key", "k2")],
        )
        .await;
    assert_eq!(refused.status, StatusCode::UNAUTHORIZED);
    let real = app
        .send(
            "POST",
            "/v1/connected-accounts",
            Some(API_KEY),
            Some(body),
            &[("idempotency-key", "k2")],
        )
        .await;
    assert_eq!(
        real.status,
        StatusCode::CREATED,
        "the 401 was never stored for this key"
    );
    assert!(real.headers.get("idempotent-replayed").is_none());
}

#[sqlx::test(migrator = "crate::db::MIGRATOR")]
async fn refuses_a_key_reused_for_another_request(pool: PgPool) {
    let app = setup_test_context(pool).await;
    let config = app.github_config().await;
    let connect = |_: &str| generate_connect_attempt(&config);
    let first = app
        .send(
            "POST",
            "/v1/connected-accounts",
            Some(API_KEY),
            Some(connect("")),
            &[("idempotency-key", "k1")],
        )
        .await;
    assert_eq!(first.status, StatusCode::CREATED);
    let other = app
        .send(
            "POST",
            "/v1/connected-accounts",
            Some(API_KEY),
            Some(connect("")),
            &[("idempotency-key", "k1")],
        )
        .await;
    assert_eq!(
        other.status,
        StatusCode::UNPROCESSABLE_ENTITY,
        "{}",
        other.body
    );
    assert_eq!(other.body["error"]["code"], "OS_08");
}

#[sqlx::test(migrator = "crate::db::MIGRATOR")]
async fn runs_a_retry_again_after_the_first_answer_was_a_server_error(pool: PgPool) {
    let app = setup_test_context(pool).await;
    let config = app.github_config().await;
    let body = generate_connect_attempt(&config);
    app.tasks.fail(true);
    let failed = app
        .send(
            "POST",
            "/v1/connected-accounts",
            Some(API_KEY),
            Some(body.clone()),
            &[("idempotency-key", "k1")],
        )
        .await;
    assert_eq!(failed.status, StatusCode::INTERNAL_SERVER_ERROR);
    app.tasks.fail(false);
    let retry = app
        .send(
            "POST",
            "/v1/connected-accounts",
            Some(API_KEY),
            Some(body),
            &[("idempotency-key", "k1")],
        )
        .await;
    assert_eq!(retry.status, StatusCode::CREATED, "{}", retry.body);
    assert!(retry.headers.get("idempotent-replayed").is_none());
}

#[sqlx::test(migrator = "crate::db::MIGRATOR")]
async fn never_replays_an_answer_that_is_not_json(pool: PgPool) {
    let app = setup_test_context(pool).await;
    let send = || async {
        let request = axum::http::Request::builder()
            .method("POST")
            .uri("/v1/connected-accounts")
            .header("content-type", "application/json")
            .header("authorization", format!("Bearer {API_KEY}"))
            .header("idempotency-key", "k1")
            .body(axum::body::Body::from("not json"))
            .unwrap();
        tower::ServiceExt::oneshot(app.api.clone(), request)
            .await
            .unwrap()
    };
    let first = send().await;
    assert!(first.status().is_client_error());
    let retry = send().await;
    assert!(retry.status().is_client_error());
    assert!(
        retry.headers().get("idempotent-replayed").is_none(),
        "the plain-text answer was not stored"
    );
}
