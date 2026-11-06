//! Tests of `choose_auth_config`.

#![allow(
    clippy::unwrap_used,
    reason = "tests: a failure here is the test failing"
)]

use super::*;
use fake::{Fake, faker::lorem::en::Word};
use serde_json::json;

fn config(provider: &str) -> (String, Value) {
    let id = format!("ac_{}", Word().fake::<String>());
    (id.clone(), json!({ "id": id, "provider": provider }))
}

#[test]
fn chooses_the_provider_s_only_app() {
    let (github, github_config) = config("github");
    let (_, linear_config) = config("linear");
    let configs = json!({ "data": [linear_config, github_config] });

    assert_eq!(choose_auth_config("github", &configs).unwrap(), github);
}

#[test]
fn says_how_to_register_one_when_the_provider_has_none() {
    let (_, linear_config) = config("linear");
    let error = choose_auth_config("github", &json!({ "data": [linear_config] })).unwrap_err();

    assert!(
        error
            .to_string()
            .contains("osiris auth-configs create --provider github")
    );
}

#[test]
fn asks_to_name_one_when_the_provider_has_several() {
    let (first, first_config) = config("github");
    let (second, second_config) = config("github");
    let error = choose_auth_config("github", &json!({ "data": [first_config, second_config] }))
        .unwrap_err();

    assert!(error.to_string().contains("--auth-config"));
    assert!(error.to_string().contains(&first) && error.to_string().contains(&second));
}
