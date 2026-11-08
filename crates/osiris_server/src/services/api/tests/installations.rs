//! A GitHub app holding its App credentials connects as an installation: no consent, active at
//! once, and its actions run with an installation token minted on demand and reused.

#![allow(
    clippy::unwrap_used,
    reason = "tests: a failure here is the test failing"
)]

use crate::testing::{ADMIN_KEY, API_KEY, TestContext, setup_test_context};
use axum::http::StatusCode;
use osiris_domain::EventType;
use osiris_providers::github::mocks::{
    MockGithubNetwork, generate_github_repository::generate_github_repository,
    mock_github_app_key::MockGithubAppKey,
};
use serde_json::{Value, json};
use sqlx::PgPool;
use wiremock::{
    Mock, ResponseTemplate,
    matchers::{header, method, path},
};

/// A GitHub app holding App credentials; answers its id.
async fn app_with_credentials(app: &TestContext) -> String {
    let answer = app
        .send(
            "POST",
            "/v1/auth-configs",
            Some(ADMIN_KEY),
            Some(json!({
                "provider": "github",
                "client_id": "an-app",
                "client_secret": "its-secret",
                "settings": { "app_id": "12345" },
                "secret_settings": { "private_key": MockGithubAppKey::default().pem() },
            })),
            &[],
        )
        .await;
    assert_eq!(answer.status, StatusCode::CREATED, "{}", answer.body);
    assert_eq!(answer.body["settings"], json!({ "app_id": "12345" }));
    assert_eq!(answer.body["has_secret_settings"], true);
    assert!(answer.body.get("secret_settings").is_none());
    answer.body["id"].as_str().unwrap().to_owned()
}

async fn connect(
    app: &TestContext,
    config: &str,
    installation: &str,
) -> crate::testing::requests::Answer {
    app.send(
        "POST",
        "/v1/connected-accounts",
        Some(API_KEY),
        Some(json!({ "auth_config_id": config, "user_id": "org_1", "installation_id": installation })),
        &[],
    )
    .await
}

async fn get_repository(app: &TestContext, id: &str) -> crate::testing::requests::Answer {
    app.send(
        "POST",
        &format!("/v1/connected-accounts/{id}/actions/github.get_repository"),
        Some(API_KEY),
        Some(json!({ "owner": "octo", "repo": "hello" })),
        &[],
    )
    .await
}

#[sqlx::test(migrator = "crate::db::MIGRATOR")]
async fn connects_as_an_installation_and_acts_with_one_minted_token(pool: PgPool) {
    let app = setup_test_context(pool).await;
    let config = app_with_credentials(&app).await;
    let github = MockGithubNetwork::on(&app.provider);
    // Minted once at connect, then reused by both actions.
    github
        .answer_installation_token("77", "ghs_installation", 1)
        .await;
    Mock::given(method("GET"))
        .and(path("/repos/octo/hello"))
        .and(header("authorization", "Bearer ghs_installation"))
        .respond_with(ResponseTemplate::new(200).set_body_json(generate_github_repository(5)))
        .expect(2)
        .mount(&app.provider)
        .await;

    let created = connect(&app, &config, "77").await;
    assert_eq!(created.status, StatusCode::CREATED, "{}", created.body);
    assert_eq!(created.body["status"], "active");
    assert_eq!(created.body["redirect_url"], Value::Null);
    let id = created.body["id"].as_str().unwrap();

    assert_eq!(get_repository(&app, id).await.status, StatusCode::OK);
    let second = get_repository(&app, id).await;
    assert_eq!(second.status, StatusCode::OK, "{}", second.body);
    assert_eq!(second.body["data"]["id"], "5");

    let account = app.account(id).await;
    assert_eq!(account["credential_kind"], "app_installation");
    assert_eq!(account["installation_id"], "77");
    assert_eq!(account["setup_complete"], true);
    assert!(
        app.events()
            .iter()
            .any(|event| event.event_type == EventType::ConnectionActive)
    );
}

#[sqlx::test(migrator = "crate::db::MIGRATOR")]
async fn mints_again_once_when_github_refuses_the_installation_s_token(pool: PgPool) {
    let app = setup_test_context(pool).await;
    let config = app_with_credentials(&app).await;
    let github = MockGithubNetwork::on(&app.provider);
    github.answer_installation_token("77", "ghs_new", 2).await;
    let created = connect(&app, &config, "77").await;
    let id = created.body["id"].as_str().unwrap().to_owned();
    // The first call is refused, the call with the newly minted token answers.
    Mock::given(method("GET"))
        .and(path("/repos/octo/hello"))
        .respond_with(
            ResponseTemplate::new(401).set_body_json(json!({ "message": "Bad credentials" })),
        )
        .up_to_n_times(1)
        .mount(&app.provider)
        .await;
    Mock::given(method("GET"))
        .and(path("/repos/octo/hello"))
        .respond_with(ResponseTemplate::new(200).set_body_json(generate_github_repository(6)))
        .mount(&app.provider)
        .await;

    let answer = get_repository(&app, &id).await;

    assert_eq!(answer.status, StatusCode::OK, "{}", answer.body);
    assert_eq!(app.account(&id).await["status"], "active");
}

#[sqlx::test(migrator = "crate::db::MIGRATOR")]
async fn refuses_an_installation_github_won_t_mint_a_token_for(pool: PgPool) {
    let app = setup_test_context(pool).await;
    let config = app_with_credentials(&app).await;
    MockGithubNetwork::on(&app.provider)
        .refuse_installation_token("404404", 404)
        .await;

    let refused = connect(&app, &config, "404404").await;

    assert_eq!(
        refused.status,
        StatusCode::FAILED_DEPENDENCY,
        "{}",
        refused.body
    );
    assert_eq!(refused.body["error"]["code"], "OS_04");
}

#[sqlx::test(migrator = "crate::db::MIGRATOR")]
async fn refuses_an_installation_through_an_app_without_credentials(pool: PgPool) {
    let app = setup_test_context(pool).await;
    let config = app.github_config().await;

    let refused = connect(&app, &config, "77").await;

    assert_eq!(refused.status, StatusCode::BAD_REQUEST, "{}", refused.body);
    assert_eq!(refused.body["error"]["code"], "OS_02");
}

#[sqlx::test(migrator = "crate::db::MIGRATOR")]
async fn refuses_settings_the_provider_does_not_read(pool: PgPool) {
    let app = setup_test_context(pool).await;

    for (provider, settings, secret_settings) in [
        (
            "github",
            json!({ "app_id": "12345" }),
            json!({ "private_key": "not a pem" }),
        ),
        (
            "github",
            json!({}),
            json!({ "private_key": "-----BEGIN RSA PRIVATE KEY-----" }),
        ),
        ("linear", json!({ "anything": true }), json!({})),
    ] {
        let refused = app
            .send(
                "POST",
                "/v1/auth-configs",
                Some(ADMIN_KEY),
                Some(json!({
                    "provider": provider,
                    "client_id": "c",
                    "client_secret": "s",
                    "settings": settings,
                    "secret_settings": secret_settings,
                })),
                &[],
            )
            .await;
        assert_eq!(
            refused.status,
            StatusCode::BAD_REQUEST,
            "{provider}: {}",
            refused.body
        );
        assert_eq!(refused.body["error"]["code"], "OS_02");
    }
}

#[sqlx::test(migrator = "crate::db::MIGRATOR")]
async fn still_requires_a_callback_for_a_consent(pool: PgPool) {
    let app = setup_test_context(pool).await;
    let config = app.github_config().await;

    let refused = app
        .send(
            "POST",
            "/v1/connected-accounts",
            Some(API_KEY),
            Some(json!({ "auth_config_id": config, "user_id": "org_1" })),
            &[],
        )
        .await;

    assert_eq!(refused.status, StatusCode::BAD_REQUEST, "{}", refused.body);
    assert!(
        refused.body["error"]["message"]
            .as_str()
            .unwrap()
            .contains("callback_url")
    );
}
