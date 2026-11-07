//! Idempotency keys only for an authenticated caller: nothing unauthenticated is stored or replayed.

use crate::testing::generate_connect_attempt::generate_connect_attempt;
use crate::testing::{ADMIN_KEY, API_KEY, setup_test_context};
use axum::http::StatusCode;
use osiris_providers::github::mocks::MockGithubNetwork;
use sqlx::PgPool;
use std::time::Duration;

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
async fn refuses_an_unreadable_body_as_os_02_and_answers_a_retry_the_same(pool: PgPool) {
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
        let answer = tower::ServiceExt::oneshot(app.api.clone(), request)
            .await
            .unwrap();
        let status = answer.status();
        let bytes = http_body_util::BodyExt::collect(answer.into_body())
            .await
            .unwrap()
            .to_bytes();
        (
            status,
            serde_json::from_slice::<serde_json::Value>(&bytes).unwrap(),
        )
    };
    let (first_status, first) = send().await;
    assert_eq!(first_status, StatusCode::BAD_REQUEST);
    assert_eq!(first["error"]["code"], "OS_02");
    let (retry_status, retry) = send().await;
    assert_eq!(retry_status, StatusCode::BAD_REQUEST);
    assert_eq!(retry, first);
}

#[sqlx::test(migrator = "crate::db::MIGRATOR")]
async fn never_replays_an_answer_that_is_not_json(pool: PgPool) {
    let app = setup_test_context(pool).await;
    // `/v1/auth-configs` has no DELETE: axum's method router answers 405 with no body, inside
    // the idempotency middleware, so the key is begun and the answer can't be stored.
    let send = || async {
        let request = axum::http::Request::builder()
            .method("DELETE")
            .uri("/v1/auth-configs")
            .header("authorization", format!("Bearer {ADMIN_KEY}"))
            .header("idempotency-key", "k1")
            .body(axum::body::Body::empty())
            .unwrap();
        tower::ServiceExt::oneshot(app.api.clone(), request)
            .await
            .unwrap()
    };
    let first = send().await;
    assert_eq!(first.status(), StatusCode::METHOD_NOT_ALLOWED);
    let retry = send().await;
    assert_eq!(retry.status(), StatusCode::METHOD_NOT_ALLOWED);
    assert!(
        retry.headers().get("idempotent-replayed").is_none(),
        "the answer that is not JSON was not stored"
    );
}

#[sqlx::test(migrator = "crate::db::MIGRATOR")]
async fn finishes_a_keyed_request_its_caller_gave_up_on(pool: PgPool) {
    let app = setup_test_context(pool).await;
    let id = app.connected(28800).await;
    MockGithubNetwork::on(&app.provider)
        .answer_revoke_after(Duration::from_millis(400), 204)
        .await;
    let path = format!("/v1/connected-accounts/{id}");
    let key = [("idempotency-key", "k1")];

    // The caller gives up once the provider has the call and is still answering.
    let provider_has_the_call = async {
        while !app
            .provider
            .received_requests()
            .await
            .unwrap()
            .iter()
            .any(|request| request.method == wiremock::http::Method::DELETE)
        {
            tokio::task::yield_now().await;
        }
    };
    tokio::select! {
        _ = app.send("DELETE", &path, Some(API_KEY), None, &key) => panic!("the caller was not cut off"),
        () = provider_has_the_call => {}
    }

    // The request still finishes, so the retry is replayed, never left answering "in flight".
    let retry = tokio::time::timeout(Duration::from_secs(10), async {
        loop {
            let answer = app.send("DELETE", &path, Some(API_KEY), None, &key).await;
            if answer.status != StatusCode::CONFLICT {
                return answer;
            }
            tokio::task::yield_now().await;
        }
    })
    .await
    .unwrap();
    assert_eq!(retry.status, StatusCode::OK, "{}", retry.body);
    assert_eq!(retry.headers["idempotent-replayed"], "true");
}

async fn refuses_a_keyed_request(pool: PgPool, key: &str, body: serde_json::Value) {
    let app = setup_test_context(pool).await;

    let answer = app
        .send(
            "POST",
            "/v1/connected-accounts",
            Some(API_KEY),
            Some(body),
            &[("idempotency-key", key)],
        )
        .await;

    assert_eq!(answer.status, StatusCode::BAD_REQUEST, "{}", answer.body);
    assert_eq!(answer.body["error"]["code"], "OS_02");
}

#[sqlx::test(migrator = "crate::db::MIGRATOR")]
async fn refuses_an_empty_idempotency_key(pool: PgPool) {
    refuses_a_keyed_request(pool, "", serde_json::json!({})).await;
}

#[sqlx::test(migrator = "crate::db::MIGRATOR")]
async fn refuses_an_idempotency_key_of_256_characters(pool: PgPool) {
    refuses_a_keyed_request(pool, &"k".repeat(256), serde_json::json!({})).await;
}

#[sqlx::test(migrator = "crate::db::MIGRATOR")]
async fn refuses_a_keyed_body_over_the_limit(pool: PgPool) {
    let oversized = serde_json::json!({ "alias": "x".repeat(1_100_000) });
    refuses_a_keyed_request(pool, "k1", oversized).await;
}

#[sqlx::test(migrator = "crate::db::MIGRATOR")]
async fn refuses_an_idempotency_key_that_is_not_visible_ascii(pool: PgPool) {
    refuses_a_keyed_request(pool, "ключ", serde_json::json!({})).await;
}
