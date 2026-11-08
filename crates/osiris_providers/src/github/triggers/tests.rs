//! GitHub's triggers: configs read strictly, deliveries refused unless signed with the trigger's
//! secret, events filtered, the webhook created or found and aligned, and a watched pull request's
//! changes seen once each.

#![allow(
    clippy::unwrap_used,
    clippy::indexing_slicing,
    reason = "tests: a failure here is the test failing"
)]

use super::{
    ON_REPO_EVENT, RepoEventConfig, WATCH_PULL_REQUEST, WatchPullRequestConfig,
    receive_delivery::receive_delivery, verify_signature::verify_signature,
};
use crate::{
    TriggerReceipt,
    github::{
        mocks::{MockGithubNetwork, generate_sha::generate_sha, github_on::github_on},
        pull_requests::mocks::generate_github_pull_request::generate_github_pull_request,
    },
};
use aws_lc_rs::hmac;
use reqwest::header::{HeaderMap, HeaderValue};
use rstest::rstest;
use serde_json::{Value, json};
use wiremock::{
    Mock, MockServer, ResponseTemplate,
    matchers::{body_partial_json, method, path},
};

const SECRET: &str = "a-trigger-secret-0123456789abcdef";

fn signed(body: &[u8], secret: &str, event: &str) -> HeaderMap {
    let key = hmac::Key::new(hmac::HMAC_SHA256, secret.as_bytes());
    let tag = hex::encode(hmac::sign(&key, body).as_ref());
    let mut headers = HeaderMap::new();
    headers.insert(
        "x-hub-signature-256",
        HeaderValue::from_str(&format!("sha256={tag}")).unwrap(),
    );
    headers.insert("x-github-event", HeaderValue::from_str(event).unwrap());
    headers.insert("x-github-delivery", HeaderValue::from_static("delivery-1"));
    headers
}

fn repo_events(events: &[&str]) -> Value {
    json!({ "owner": "octo", "repo": "hello", "events": events })
}

#[rstest]
#[case::no_events(json!({ "owner": "octo", "repo": "hello", "events": [] }), "at least one")]
#[case::an_unknown_event(repo_events(&["pushes"]), "pushes")]
#[case::a_bad_owner(json!({ "owner": "-octo", "repo": "hello", "events": ["push"] }), "owner")]
#[case::a_dot_repo(json!({ "owner": "octo", "repo": "..", "events": ["push"] }), "repo")]
#[case::an_unknown_field(json!({ "owner": "octo", "repo": "hello", "events": ["push"], "x": 1 }), "x")]
fn refuses_a_repo_event_config_that_cannot_work(#[case] config: Value, #[case] named: &str) {
    let Err(reason) = RepoEventConfig::read(&config) else {
        panic!("the config was accepted");
    };
    assert!(reason.contains(named), "{reason}");
}

#[test]
fn refuses_a_watch_of_pull_request_zero() {
    assert!(
        WatchPullRequestConfig::read(&json!({ "owner": "octo", "repo": "hello", "number": 0 }))
            .is_err()
    );
    assert!(
        WatchPullRequestConfig::read(&json!({ "owner": "octo", "repo": "hello", "number": 7 }))
            .is_ok()
    );
}

#[test]
fn checks_the_signature_with_the_trigger_s_secret() {
    let body = br#"{"zen":"hi"}"#;
    let right = signed(body, SECRET, "push");
    let wrong = signed(body, "another-secret", "push");
    let header = |headers: &HeaderMap| headers["x-hub-signature-256"].to_str().unwrap().to_owned();

    assert!(verify_signature(SECRET, body, Some(&header(&right))));
    assert!(!verify_signature(SECRET, body, Some(&header(&wrong))));
    assert!(!verify_signature(
        SECRET,
        b"tampered",
        Some(&header(&right))
    ));
    assert!(!verify_signature(SECRET, body, None));
    assert!(!verify_signature(SECRET, body, Some("sha1=abc")));
}

#[test]
fn fires_on_a_watched_event_with_github_s_payload() {
    let body = br#"{"ref":"refs/heads/main"}"#;

    let receipt = receive_delivery(
        ON_REPO_EVENT,
        &repo_events(&["push"]),
        &signed(body, SECRET, "push"),
        body,
        SECRET,
    );

    let TriggerReceipt::Fired { delivery_id, data } = receipt else {
        panic!("it did not fire: {receipt:?}");
    };
    assert_eq!(delivery_id.as_deref(), Some("delivery-1"));
    assert_eq!(data["event"], "push");
    assert_eq!(data["payload"]["ref"], "refs/heads/main");
}

#[rstest]
#[case::unsigned("push", "another-secret", "push", false)]
#[case::a_ping("ping", SECRET, "push", true)]
#[case::an_unwatched_event("issues", SECRET, "push", true)]
fn does_not_fire_on_what_the_trigger_does_not_take(
    #[case] event: &str,
    #[case] signed_with: &str,
    #[case] watched: &str,
    #[case] signed_right: bool,
) {
    let body = b"{}";
    let receipt = receive_delivery(
        ON_REPO_EVENT,
        &repo_events(&[watched]),
        &signed(body, signed_with, event),
        body,
        SECRET,
    );
    if signed_right {
        assert!(matches!(receipt, TriggerReceipt::Ignored(_)), "{receipt:?}");
    } else {
        assert_eq!(receipt, TriggerReceipt::Unsigned);
    }
}

#[test]
fn fires_on_every_event_for_a_star() {
    let body = br#"{"action":"opened"}"#;
    let receipt = receive_delivery(
        ON_REPO_EVENT,
        &repo_events(&["*"]),
        &signed(body, SECRET, "issues"),
        body,
        SECRET,
    );
    assert!(matches!(receipt, TriggerReceipt::Fired { .. }));
}

#[test]
fn checks_a_watched_pull_request_only_on_its_own_deliveries() {
    let config = json!({ "owner": "octo", "repo": "hello", "number": 7 });
    let ours = br#"{"number":7,"action":"synchronize"}"#;
    let another = br#"{"number":8,"action":"synchronize"}"#;

    let checked = receive_delivery(
        WATCH_PULL_REQUEST,
        &config,
        &signed(ours, SECRET, "pull_request"),
        ours,
        SECRET,
    );
    let ignored = receive_delivery(
        WATCH_PULL_REQUEST,
        &config,
        &signed(another, SECRET, "pull_request"),
        another,
        SECRET,
    );

    assert_eq!(checked, TriggerReceipt::Check);
    assert!(matches!(ignored, TriggerReceipt::Ignored(_)));
}

#[tokio::test]
async fn creates_the_webhook_delivering_to_the_trigger() {
    let server = MockServer::start().await;
    Mock::given(method("POST"))
        .and(path("/repos/octo/hello/hooks"))
        .and(body_partial_json(json!({
            "name": "web",
            "events": ["push"],
            "config": { "url": "https://osiris.test/webhooks/triggers/tr_1", "secret": SECRET, "content_type": "json", "insecure_ssl": "0" },
        })))
        .respond_with(ResponseTemplate::new(201).set_body_json(json!({ "id": 99 })))
        .expect(1)
        .mount(&server)
        .await;

    let id = github_on(&server)
        .create_hook(
            &reqwest::Client::new(),
            "token",
            "octo",
            "hello",
            "https://osiris.test/webhooks/triggers/tr_1",
            SECRET,
            &["push".to_owned()],
            false,
        )
        .await
        .unwrap();

    assert_eq!(id, "99");
}

#[tokio::test]
async fn aligns_the_webhook_github_says_already_exists() {
    let server = MockServer::start().await;
    let url = "https://osiris.test/webhooks/triggers/tr_1";
    Mock::given(method("POST"))
        .and(path("/repos/octo/hello/hooks"))
        .respond_with(
            ResponseTemplate::new(422)
                .set_body_json(json!({ "message": "Hook already exists on this repository" })),
        )
        .mount(&server)
        .await;
    MockGithubNetwork::on(&server)
        .answer(
            "GET",
            "/repos/octo/hello/hooks",
            &[("page", "1"), ("per_page", "100")],
            200,
            json!([{ "id": 5, "config": { "url": "https://elsewhere.test" } }, { "id": 6, "config": { "url": url } }]),
        )
        .await;
    Mock::given(method("PATCH"))
        .and(path("/repos/octo/hello/hooks/6"))
        .respond_with(ResponseTemplate::new(200).set_body_json(json!({ "id": 6 })))
        .expect(1)
        .mount(&server)
        .await;

    let id = github_on(&server)
        .create_hook(
            &reqwest::Client::new(),
            "token",
            "octo",
            "hello",
            url,
            SECRET,
            &["push".to_owned()],
            false,
        )
        .await
        .unwrap();

    assert_eq!(id, "6");
}

#[tokio::test]
async fn deletes_a_webhook_or_counts_one_already_gone_as_deleted() {
    let server = MockServer::start().await;
    let network = MockGithubNetwork::on(&server);
    network
        .answer_empty("DELETE", "/repos/octo/hello/hooks/6", 204)
        .await;
    network
        .answer(
            "DELETE",
            "/repos/octo/hello/hooks/7",
            &[],
            404,
            json!({ "message": "Not Found" }),
        )
        .await;
    let github = github_on(&server);
    let http = reqwest::Client::new();

    github
        .delete_hook(&http, "token", "octo", "hello", "6")
        .await
        .unwrap();
    github
        .delete_hook(&http, "token", "octo", "hello", "7")
        .await
        .unwrap();
}

#[tokio::test]
async fn sees_each_change_to_a_watched_pull_request_once() {
    let server = MockServer::start().await;
    let mut pull_request = generate_github_pull_request(70, 7);
    let config = WatchPullRequestConfig {
        owner: "octo".into(),
        repo: "hello".into(),
        number: 7,
    };
    let github = github_on(&server);
    let http = reqwest::Client::new();
    let answer = |body: Value| ResponseTemplate::new(200).set_body_json(body);

    let first_mock = Mock::given(method("GET"))
        .and(path("/repos/octo/hello/pulls/7"))
        .respond_with(answer(pull_request.clone()))
        .mount_as_scoped(&server)
        .await;
    // The first check records the state, firing nothing.
    let first = github
        .check_pull_request(&http, "token", &config, None)
        .await
        .unwrap();
    assert!(first.fired.is_none());
    // The same state again fires nothing.
    let same = github
        .check_pull_request(&http, "token", &config, Some(&first.checkpoint))
        .await
        .unwrap();
    assert!(same.fired.is_none());
    assert_eq!(same.checkpoint, first.checkpoint);
    drop(first_mock);

    pull_request["head"]["sha"] = json!(generate_sha());
    Mock::given(method("GET"))
        .and(path("/repos/octo/hello/pulls/7"))
        .respond_with(answer(pull_request))
        .mount(&server)
        .await;
    let changed = github
        .check_pull_request(&http, "token", &config, Some(&first.checkpoint))
        .await
        .unwrap();

    let (id, data) = changed.fired.unwrap();
    assert!(id.starts_with("1-"), "{id}");
    assert_eq!(data["pull_request"]["number"], 7);
    assert_eq!(changed.checkpoint["sequence"], 1);
    assert_ne!(changed.checkpoint["version"], first.checkpoint["version"]);
}
