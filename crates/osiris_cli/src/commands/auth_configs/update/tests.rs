//! Tests of `update`.

use super::*;
use fake::{Fake, faker::internet::en::Password};
use serde_json::json;

fn update(client_id: Option<&str>, client_secret: Option<&str>, scopes: &[&str]) -> Update {
    Update {
        id: "ac_1".into(),
        client_id: client_id.map(str::to_owned),
        client_secret: client_secret.map(str::to_owned),
        scopes: scopes.iter().map(|scope| (*scope).to_owned()).collect(),
    }
}

#[test]
fn sends_nothing_for_a_flag_left_out() {
    assert_eq!(update(None, None, &[]).body(), json!({}));
}

#[test]
fn never_sends_an_empty_scopes_list_that_would_wipe_the_app_s_scopes() {
    let client_id: String = Password(8..16).fake();

    let body = update(Some(&client_id), None, &[]).body();

    assert_eq!(body, json!({ "client_id": client_id }));
}

#[test]
fn sends_each_value_given() {
    let client_id: String = Password(8..16).fake();
    let client_secret: String = Password(32..40).fake();

    let body = update(Some(&client_id), Some(&client_secret), &["repo", "user"]).body();

    assert_eq!(
        body,
        json!({ "client_id": client_id, "client_secret": client_secret, "scopes": ["repo", "user"] })
    );
}
