//! The conformance checks fail on what the spec doesn't promise: a missing required field, a wrong
//! type, an undocumented status, an event out of shape. Each passes what it does promise.

use super::{assert_answer_conforms, assert_event_conforms};
use axum::http::{HeaderMap, StatusCode};
use osiris_versioning::RELEASES;
use serde_json::{Value, json};

fn answer(method: &str, path: &str, status: StatusCode, body: &Value) {
    assert_answer_conforms(RELEASES, method, path, status, &HeaderMap::new(), body);
}

fn auth_config() -> Value {
    json!({
        "id": "ac_1", "provider": "github", "client_id": "c", "scopes": [],
        "settings": {}, "has_secret_settings": false,
        "created_at": "2026-11-04T00:00:00Z", "updated_at": "2026-11-04T00:00:00Z",
    })
}

fn event() -> Value {
    json!({
        "version": 1, "id": "msg_1", "occurred_at": "2026-11-04T00:00:00Z",
        "osiris_version": "2026-11-04.anubis", "event_id": "ev_1",
        "type": "connection.active", "connected_account_id": "ca_1",
    })
}

#[test]
fn passes_what_the_spec_promises() {
    answer(
        "GET",
        "/v1/auth-configs/ac_1",
        StatusCode::OK,
        &auth_config(),
    );
    answer(
        "GET",
        "/v1/auth-configs?limit=1",
        StatusCode::OK,
        &json!({ "data": [auth_config()], "next_cursor": null, "previous_cursor": null }),
    );
    assert_event_conforms(RELEASES, &event());
}

#[test]
#[should_panic(expected = "\"client_id\" is a required property")]
fn fails_an_answer_missing_a_required_field() {
    let mut body = auth_config();
    body.as_object_mut().unwrap().remove("client_id");
    answer("GET", "/v1/auth-configs/ac_1", StatusCode::OK, &body);
}

#[test]
#[should_panic(expected = "is not of type \"array\"")]
fn fails_an_answer_of_the_wrong_type() {
    answer(
        "GET",
        "/v1/auth-configs",
        StatusCode::OK,
        &json!({ "data": "none", "next_cursor": null, "previous_cursor": null }),
    );
}

#[test]
#[should_panic(expected = "doesn't document 418")]
fn fails_an_undocumented_status() {
    answer(
        "GET",
        "/v1/auth-configs/ac_1",
        StatusCode::IM_A_TEAPOT,
        &json!({}),
    );
}

#[test]
#[should_panic(expected = "\"connected_account_id\" is a required property")]
fn fails_an_event_out_of_shape() {
    let mut body = event();
    body.as_object_mut().unwrap().remove("connected_account_id");
    assert_event_conforms(RELEASES, &body);
}
