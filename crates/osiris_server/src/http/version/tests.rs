//! Two releases served at once from one current model: amun's requests brought up, anubis's
//! answers taken back, and each answer naming the release and, when superseded, its sunset.

use super::{lifecycle_headers::lifecycle_headers, mocks::MOCK_RELEASES};
use crate::{
    services::api::versioned_spec,
    testing::{ADMIN_KEY, setup_test_context},
};
use axum::http::{HeaderMap, StatusCode};
use osiris_api_models::version::translate::event_down;
use osiris_events::{
    Message, MessageBus, adapters::memory::MemoryTransport,
    testing::generate_event_message::generate_event_message,
};
use serde_json::json;
use sqlx::PgPool;
use std::sync::Arc;
use time::Duration;

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
    let spec = versioned_spec(MOCK_RELEASES, amun);
    let properties = &spec["components"]["schemas"]["AuthConfig"]["properties"];
    assert!(properties.get("app_id").is_some());
    assert!(properties.get("client_id").is_none());
    assert_eq!(spec["info"]["version"], AMUN);

    let mut event = json!({ "type": "connection.active", "connected_account_id": "ca_1" });
    event_down(MOCK_RELEASES, amun, &mut event);
    assert_eq!(
        event,
        json!({ "type": "connection.active", "connection_id": "ca_1" })
    );
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
