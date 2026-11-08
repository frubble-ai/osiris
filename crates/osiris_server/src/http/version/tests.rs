//! Two releases served at once from one current model: amun's requests brought up, anubis's
//! answers taken back, and each answer naming the release and, when superseded, its sunset.

use super::lifecycle_headers::lifecycle_headers;
use crate::{
    services::api::versioned_spec,
    testing::{
        ADMIN_KEY, API_KEY, generate_connect_attempt::generate_connect_attempt,
        mock_releases::MOCK_RELEASES, setup_test_context, setup_test_context_with,
    },
};
use axum::{
    body::Body,
    extract::Request,
    http::{HeaderMap, StatusCode},
};
use http_body_util::BodyExt;
use osiris_events::{
    Message, MessageBus, adapters::memory::MemoryTransport,
    testing::generate_event_message::generate_event_message,
};
use osiris_versioning::translate::event_down;
use serde_json::json;
use sqlx::PgPool;
use std::sync::Arc;
use time::Duration;
use tower::ServiceExt;

const AMUN: &str = "2026-01-01.amun";
const ANUBIS: &str = "2026-11-04.anubis";

#[sqlx::test(migrator = "crate::db::MIGRATOR")]
async fn serves_an_older_release_its_own_shape_beside_the_current_one(pool: PgPool) {
    let app = setup_test_context(pool).await.serving(MOCK_RELEASES);

    let created = app
        .send(
            "POST",
            "/v1/auth-configs",
            Some(ADMIN_KEY),
            Some(json!({ "provider": "github", "app_id": "the-app", "client_secret": "s" })),
            &[("osiris-version", AMUN)],
        )
        .await;
    assert_eq!(created.status, StatusCode::CREATED, "{}", created.body);
    assert_eq!(created.headers["osiris-version"], AMUN);
    assert_eq!(created.body["app_id"], "the-app");
    assert!(created.body.get("client_id").is_none());

    let id = created.body["id"].as_str().expect("an id");
    let current = app
        .send(
            "GET",
            &format!("/v1/auth-configs/{id}"),
            Some(ADMIN_KEY),
            None,
            &[("osiris-version", ANUBIS)],
        )
        .await;
    assert_eq!(current.headers["osiris-version"], ANUBIS);
    assert_eq!(current.body["client_id"], "the-app");
    assert!(current.body.get("app_id").is_none());

    let listed = app
        .send(
            "GET",
            "/v1/auth-configs",
            Some(ADMIN_KEY),
            None,
            &[("osiris-version", AMUN)],
        )
        .await;
    assert_eq!(listed.body["data"][0]["app_id"], "the-app");
}

#[sqlx::test(migrator = "crate::db::MIGRATOR")]
async fn serves_a_request_naming_no_version_at_the_pinned_release_even_an_older_one(pool: PgPool) {
    let app = setup_test_context(pool)
        .await
        .serving_pinned(MOCK_RELEASES, AMUN);
    app.github_config().await;

    // No Osiris-Version: served at amun, the pin, in amun's shape and marked as superseded.
    let pinned = app
        .send("GET", "/v1/auth-configs", Some(ADMIN_KEY), None, &[])
        .await;

    assert_eq!(pinned.status, StatusCode::OK, "{}", pinned.body);
    assert_eq!(pinned.headers["osiris-version"], AMUN);
    assert!(pinned.body["data"][0].get("app_id").is_some());
    assert!(pinned.body["data"][0].get("client_id").is_none());
    assert!(pinned.headers.get("deprecation").is_some());
    assert!(pinned.headers.get("sunset").is_some());
}

#[sqlx::test(migrator = "crate::db::MIGRATOR")]
async fn marks_a_superseded_release_s_answers_with_its_deprecation_and_sunset(pool: PgPool) {
    let app = setup_test_context(pool).await.serving(MOCK_RELEASES);

    let old = app
        .send(
            "GET",
            "/v1/auth-configs",
            Some(ADMIN_KEY),
            None,
            &[("osiris-version", AMUN)],
        )
        .await;
    let current = app
        .send(
            "GET",
            "/v1/auth-configs",
            Some(ADMIN_KEY),
            None,
            &[("osiris-version", ANUBIS)],
        )
        .await;

    // anubis came out on 2026-11-04; the fixture's support window is the default, 730 days.
    assert_eq!(old.headers["deprecation"], "@1793750400");
    assert_eq!(old.headers["sunset"], "Fri, 03 Nov 2028 00:00:00 GMT");
    assert!(current.headers.get("deprecation").is_none());
    assert!(current.headers.get("sunset").is_none());
    assert_eq!(old.headers["osiris-stability"], "stable");
    assert_eq!(current.headers["osiris-stability"], "preview");
}

#[sqlx::test(migrator = "crate::db::MIGRATOR")]
async fn leaves_an_older_release_s_errors_and_untouched_routes_as_they_are(pool: PgPool) {
    let app = setup_test_context(pool).await.serving(MOCK_RELEASES);

    let missing = app
        .send(
            "GET",
            "/v1/auth-configs/ac_missing",
            Some(ADMIN_KEY),
            None,
            &[("osiris-version", AMUN)],
        )
        .await;

    assert_eq!(missing.status, StatusCode::NOT_FOUND);
    assert_eq!(missing.body["error"]["code"], "OS_01");
    assert_eq!(missing.headers["osiris-version"], AMUN);
}

#[test]
fn takes_the_spec_and_events_back_to_the_older_release() {
    let amun = &MOCK_RELEASES[0];
    let spec = versioned_spec(MOCK_RELEASES, amun).unwrap();
    let properties = &spec["components"]["schemas"]["AuthConfig"]["properties"];
    assert!(properties.get("app_id").is_some());
    assert!(properties.get("client_id").is_none());
    assert_eq!(spec["info"]["version"], AMUN);
    // amun's clients were promised anubis's date plus the default 730 days.
    assert_eq!(spec["info"]["x-osiris-sunset"], "2028-11-03");
    assert!(
        versioned_spec(MOCK_RELEASES, &MOCK_RELEASES[1]).unwrap()["info"]
            .get("x-osiris-sunset")
            .is_none()
    );

    let mut event = json!({ "type": "connection.active", "connected_account_id": "ca_1" });
    event_down(MOCK_RELEASES, amun, &mut event);
    assert_eq!(
        event,
        json!({ "type": "connection.active", "connection_id": "ca_1" })
    );
}

#[sqlx::test(migrator = "crate::db::MIGRATOR")]
async fn brings_an_older_release_s_query_up_to_the_current_one(pool: PgPool) {
    let app = setup_test_context(pool).await.serving(MOCK_RELEASES);
    let config = app.github_config().await;
    let attempt = generate_connect_attempt(&config);
    let (id, _) = app.start_connect_with(&attempt).await;
    let alias = attempt["alias"].as_str().expect("an alias");

    let found = app
        .send(
            "GET",
            &format!("/v1/connected-accounts?label={alias}"),
            Some(API_KEY),
            None,
            &[("osiris-version", AMUN)],
        )
        .await;
    let missed = app
        .send(
            "GET",
            "/v1/connected-accounts?label=no-such-alias",
            Some(API_KEY),
            None,
            &[("osiris-version", AMUN)],
        )
        .await;

    assert_eq!(found.status, StatusCode::OK, "{}", found.body);
    assert_eq!(found.body["data"][0]["id"], id.as_str());
    assert_eq!(missed.body["data"], json!([]));
}

#[sqlx::test(migrator = "crate::db::MIGRATOR")]
async fn applies_an_older_release_s_default_to_a_request_without_a_query(pool: PgPool) {
    let app = setup_test_context(pool).await.serving(MOCK_RELEASES);
    app.github_config().await;
    app.auth_config("linear").await;

    let old = app
        .send(
            "GET",
            "/v1/auth-configs",
            Some(ADMIN_KEY),
            None,
            &[("osiris-version", AMUN)],
        )
        .await;
    let current = app
        .send(
            "GET",
            "/v1/auth-configs",
            Some(ADMIN_KEY),
            None,
            &[("osiris-version", ANUBIS)],
        )
        .await;

    // amun listed one auth config a page when asked for no limit.
    assert_eq!(old.body["data"].as_array().map(Vec::len), Some(1));
    assert_eq!(current.body["data"].as_array().map(Vec::len), Some(2));
}

#[sqlx::test(migrator = "crate::db::MIGRATOR")]
async fn passes_an_older_release_s_unreadable_body_on_for_the_route_to_refuse(pool: PgPool) {
    let app = setup_test_context(pool).await.serving(MOCK_RELEASES);

    let refused = app
        .send_raw(
            "POST",
            "/v1/auth-configs",
            Some(ADMIN_KEY),
            "not json".into(),
            &[("osiris-version", AMUN)],
        )
        .await;

    assert_eq!(refused.status, StatusCode::BAD_REQUEST);
    assert_eq!(refused.body["error"]["code"], "OS_02");
    assert_eq!(
        refused.body["error"]["message"],
        "the body is not valid JSON"
    );
}

#[sqlx::test(migrator = "crate::db::MIGRATOR")]
async fn passes_an_older_release_s_answer_that_is_not_json_on_untouched(pool: PgPool) {
    let app = setup_test_context(pool).await.serving(MOCK_RELEASES);
    // `/v1/auth-configs` has no DELETE: axum answers 405 with no body, which nothing translates.
    let request = Request::builder()
        .method("DELETE")
        .uri("/v1/auth-configs")
        .header("authorization", format!("Bearer {ADMIN_KEY}"))
        .header("osiris-version", AMUN)
        .body(Body::empty())
        .unwrap();

    let answer = app.api.clone().oneshot(request).await.unwrap();

    assert_eq!(answer.status(), StatusCode::METHOD_NOT_ALLOWED);
    assert_eq!(answer.headers()["osiris-version"], AMUN);
    let bytes = answer.into_body().collect().await.unwrap().to_bytes();
    assert!(bytes.is_empty());
}

#[sqlx::test(migrator = "crate::db::MIGRATOR")]
async fn refuses_a_body_too_large_the_same_at_every_release(pool: PgPool) {
    let app = setup_test_context(pool).await.serving(MOCK_RELEASES);
    let too_large = "x".repeat(app.config.settings.server.max_body() + 1);

    for release in [AMUN, ANUBIS] {
        let refused = app
            .send_raw(
                "POST",
                "/v1/auth-configs",
                Some(ADMIN_KEY),
                too_large.clone(),
                &[("osiris-version", release)],
            )
            .await;
        assert_eq!(
            refused.status,
            StatusCode::PAYLOAD_TOO_LARGE,
            "at {release}"
        );
        assert_eq!(refused.body["error"]["code"], "OS_10", "at {release}");
        assert_eq!(refused.headers["osiris-version"], release);
    }
}

#[sqlx::test(migrator = "crate::db::MIGRATOR")]
async fn answers_500_rather_than_an_older_release_s_answer_it_cannot_read_whole(pool: PgPool) {
    let app = setup_test_context_with(pool, |config| config.settings.server.max_answer_bytes = 8)
        .await
        .serving(MOCK_RELEASES);

    let old = app
        .send(
            "GET",
            "/v1/auth-configs",
            Some(ADMIN_KEY),
            None,
            &[("osiris-version", AMUN)],
        )
        .await;
    let current = app
        .send(
            "GET",
            "/v1/auth-configs",
            Some(ADMIN_KEY),
            None,
            &[("osiris-version", ANUBIS)],
        )
        .await;

    assert_eq!(old.status, StatusCode::INTERNAL_SERVER_ERROR);
    assert_eq!(old.body["error"]["code"], "OS_99");
    assert_eq!(old.headers["osiris-version"], AMUN);
    // The current release's answer is never read whole, so the limit doesn't touch it.
    assert_eq!(current.status, StatusCode::OK);
}

#[tokio::test]
async fn writes_events_in_the_pinned_release() {
    let transport = Arc::new(MemoryTransport::default());
    let bus = MessageBus::new(transport.clone()).written_in(MOCK_RELEASES, &MOCK_RELEASES[0]);
    let event = generate_event_message();

    bus.send(Message::Event(event.clone()))
        .await
        .expect("published");

    let sent = transport.sent().pop().expect("one message");
    let body: serde_json::Value = serde_json::from_slice(&sent.data).expect("JSON");
    assert_eq!(body["osiris_version"], AMUN);
    assert_eq!(body["connection_id"], event.connected_account_id);
    assert!(body.get("connected_account_id").is_none());
    assert!(
        sent.attributes
            .contains(&("osiris_version".to_owned(), AMUN.to_owned()))
    );
}

#[test]
fn marks_nothing_on_the_current_release() {
    let mut headers = HeaderMap::new();
    lifecycle_headers(
        MOCK_RELEASES,
        &MOCK_RELEASES[1],
        Duration::days(730),
        &mut headers,
    );
    assert!(headers.is_empty());
}
