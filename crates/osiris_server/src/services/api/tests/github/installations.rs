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
    MockGithubNetwork,
    generate_github_names::{generate_github_login, generate_github_repo, generate_github_token},
    generate_github_repository::generate_github_repository,
    mock_github_app_key::MockGithubAppKey,
};
use serde_json::{Value, json};
use sqlx::PgPool;

/// A GitHub app holding App credentials; answers its id.
pub(super) async fn app_with_credentials(app: &TestContext) -> String {
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

pub(super) async fn connect(
    app: &TestContext,
    config: &str,
    installation: &str,
) -> crate::testing::requests::Answer {
    app.send(
        "POST",
        "/v1/connected-accounts",
        Some(ADMIN_KEY),
        Some(json!({ "auth_config_id": config, "user_id": "org_1", "installation_id": installation })),
        &[],
    )
    .await
}

/// An installation connection through an app holding App credentials, its token minted once; answers
/// its id.
async fn installation_connection(app: &TestContext) -> String {
    let config = app_with_credentials(app).await;
    MockGithubNetwork::on(&app.provider)
        .answer_installation_token("77", &generate_github_token(), 1)
        .await;
    let created = connect(app, &config, "77").await;
    assert_eq!(created.status, StatusCode::CREATED, "{}", created.body);
    created.body["id"].as_str().unwrap().to_owned()
}

async fn run(
    app: &TestContext,
    id: &str,
    action: &str,
    input: Option<Value>,
) -> crate::testing::requests::Answer {
    app.send(
        "POST",
        &format!("/v1/connected-accounts/{id}/actions/{action}"),
        Some(API_KEY),
        input,
        &[],
    )
    .await
}

/// The connection's `github.get_repository` of `owner/repo`.
async fn get_repository(
    app: &TestContext,
    id: &str,
    (owner, repo): (&str, &str),
) -> crate::testing::requests::Answer {
    app.send(
        "POST",
        &format!("/v1/connected-accounts/{id}/actions/github.get_repository"),
        Some(API_KEY),
        Some(json!({ "owner": owner, "repo": repo })),
        &[],
    )
    .await
}

#[sqlx::test(migrator = "crate::db::MIGRATOR")]
async fn connects_as_an_installation_and_acts_with_one_minted_token(pool: PgPool) {
    let app = setup_test_context(pool).await;
    let config = app_with_credentials(&app).await;
    let (owner, repo, token) = (
        generate_github_login(),
        generate_github_repo(),
        generate_github_token(),
    );
    let github = MockGithubNetwork::on(&app.provider);
    // Minted once at connect, then reused by both actions.
    github.answer_installation_token("77", &token, 1).await;
    github
        .answer_for_token(
            &format!("/repos/{owner}/{repo}"),
            &token,
            generate_github_repository(5),
            2,
        )
        .await;

    let created = connect(&app, &config, "77").await;
    assert_eq!(created.status, StatusCode::CREATED, "{}", created.body);
    assert_eq!(created.body["status"], "active");
    assert_eq!(created.body["redirect_url"], Value::Null);
    let id = created.body["id"].as_str().unwrap();

    assert_eq!(
        get_repository(&app, id, (&owner, &repo)).await.status,
        StatusCode::OK
    );
    let second = get_repository(&app, id, (&owner, &repo)).await;
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
async fn mints_one_token_for_actions_that_miss_the_cache_at_once(pool: PgPool) {
    let app = setup_test_context(pool).await;
    let config = app_with_credentials(&app).await;
    let (owner, repo, token) = (
        generate_github_login(),
        generate_github_repo(),
        generate_github_token(),
    );
    let github = MockGithubNetwork::on(&app.provider);
    // Minted at connect and once more after the cache is cleared: never once per action.
    github.answer_installation_token("77", &token, 2).await;
    github
        .answer_for_token(
            &format!("/repos/{owner}/{repo}"),
            &token,
            generate_github_repository(5),
            4,
        )
        .await;
    let created = connect(&app, &config, "77").await;
    let id = created.body["id"].as_str().unwrap().to_owned();
    // Saving the app's credentials again drops the token, so every action below misses it.
    let saved = app
        .send(
            "PATCH",
            &format!("/v1/auth-configs/{config}"),
            Some(ADMIN_KEY),
            Some(
                json!({ "secret_settings": { "private_key": MockGithubAppKey::default().pem() } }),
            ),
            &[],
        )
        .await;
    assert_eq!(saved.status, StatusCode::OK, "{}", saved.body);

    let answers =
        futures::future::join_all((0..4).map(|_| get_repository(&app, &id, (&owner, &repo)))).await;

    for answer in answers {
        assert_eq!(answer.status, StatusCode::OK, "{}", answer.body);
    }
}

#[sqlx::test(migrator = "crate::db::MIGRATOR")]
async fn mints_again_once_when_github_refuses_the_installation_s_token(pool: PgPool) {
    let app = setup_test_context(pool).await;
    let config = app_with_credentials(&app).await;
    let (owner, repo) = (generate_github_login(), generate_github_repo());
    let route = format!("/repos/{owner}/{repo}");
    let github = MockGithubNetwork::on(&app.provider);
    github
        .answer_installation_token("77", &generate_github_token(), 2)
        .await;
    let created = connect(&app, &config, "77").await;
    let id = created.body["id"].as_str().unwrap().to_owned();
    // The first call is refused, the call with the newly minted token answers.
    github
        .answer_once("GET", &route, 401, json!({ "message": "Bad credentials" }))
        .await;
    github
        .answer_times("GET", &route, 200, generate_github_repository(6), 1)
        .await;

    let answer = get_repository(&app, &id, (&owner, &repo)).await;

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

    for (settings, secret_settings) in [
        (
            json!({ "app_id": "12345" }),
            json!({ "private_key": "not a pem" }),
        ),
        (
            json!({}),
            json!({ "private_key": "-----BEGIN RSA PRIVATE KEY-----" }),
        ),
    ] {
        let refused = app
            .send(
                "POST",
                "/v1/auth-configs",
                Some(ADMIN_KEY),
                Some(json!({
                    "provider": "github",
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
            "{settings}: {}",
            refused.body
        );
        assert_eq!(refused.body["error"]["code"], "OS_02");
    }
}

#[sqlx::test(migrator = "crate::db::MIGRATOR")]
async fn refuses_an_installation_connection_made_with_the_api_key(pool: PgPool) {
    let app = setup_test_context(pool).await;
    let config = app_with_credentials(&app).await;
    // No token may be minted: the key is refused before GitHub is asked.
    MockGithubNetwork::on(&app.provider)
        .answer_installation_token("77", &generate_github_token(), 0)
        .await;

    let refused = app
        .send(
            "POST",
            "/v1/connected-accounts",
            Some(API_KEY),
            Some(json!({ "auth_config_id": config, "user_id": "org_1", "installation_id": "77" })),
            &[],
        )
        .await;

    assert_eq!(refused.status, StatusCode::UNAUTHORIZED, "{}", refused.body);
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

#[sqlx::test(migrator = "crate::db::MIGRATOR")]
async fn refuses_an_action_that_acts_as_a_user_before_github_is_called(pool: PgPool) {
    let app = setup_test_context(pool).await;
    let id = installation_connection(&app).await;
    MockGithubNetwork::on(&app.provider)
        .expect_nothing_at("/user")
        .await;

    let refused = run(&app, &id, "github.get_current_user", None).await;

    assert_eq!(refused.status, StatusCode::BAD_REQUEST, "{}", refused.body);
    assert_eq!(refused.body["error"]["code"], "OS_02");
    assert!(
        refused.body["error"]["message"]
            .as_str()
            .unwrap()
            .contains("acts as a user"),
        "{}",
        refused.body
    );
    // Only the installation's token was minted; nothing else reached GitHub.
    let requests = app.provider.received_requests().await.unwrap();
    assert!(
        requests
            .iter()
            .all(|request| request.url.path() == "/app/installations/77/access_tokens"),
        "{:?}",
        requests
            .iter()
            .map(|request| request.url.path())
            .collect::<Vec<_>>()
    );
}

#[sqlx::test(migrator = "crate::db::MIGRATOR")]
async fn lists_my_repositories_as_the_installation_s_repositories(pool: PgPool) {
    let app = setup_test_context(pool).await;
    let id = installation_connection(&app).await;
    let repository = generate_github_repository(8);
    let github = MockGithubNetwork::on(&app.provider);
    github.expect_nothing_at("/user/repos").await;
    github
        .answer(
            "GET",
            "/installation/repositories",
            &[],
            200,
            json!({ "total_count": 1, "repositories": [repository.clone()] }),
        )
        .await;

    let answer = run(&app, &id, "github.list_my_repositories", Some(json!({}))).await;

    assert_eq!(answer.status, StatusCode::OK, "{}", answer.body);
    assert_eq!(answer.body["data"][0]["id"], "8");
    assert_eq!(answer.body["data"][0]["full_name"], repository["full_name"]);
}

#[sqlx::test(migrator = "crate::db::MIGRATOR")]
async fn refuses_a_user_s_filter_when_listing_the_installation_s_repositories(pool: PgPool) {
    let app = setup_test_context(pool).await;
    let id = installation_connection(&app).await;
    let github = MockGithubNetwork::on(&app.provider);
    github.expect_nothing_at("/installation/repositories").await;
    github.expect_nothing_at("/user/repos").await;

    let refused = run(
        &app,
        &id,
        "github.list_my_repositories",
        Some(json!({ "visibility": "private" })),
    )
    .await;

    assert_eq!(refused.status, StatusCode::BAD_REQUEST, "{}", refused.body);
    assert_eq!(refused.body["error"]["code"], "OS_02");
    assert!(
        refused.body["error"]["message"]
            .as_str()
            .unwrap()
            .contains("visibility"),
        "{}",
        refused.body
    );
}

#[sqlx::test(migrator = "crate::db::MIGRATOR")]
async fn lists_repositories_as_the_installation_s_repositories(pool: PgPool) {
    let app = setup_test_context(pool).await;
    let id = installation_connection(&app).await;
    let repository = generate_github_repository(4);
    let github = MockGithubNetwork::on(&app.provider);
    github.expect_nothing_at("/user/installations").await;
    github
        .answer(
            "GET",
            "/installation/repositories",
            &[],
            200,
            json!({ "total_count": 1, "repositories": [repository.clone()] }),
        )
        .await;

    let answer = run(&app, &id, "github.list_repositories", None).await;

    assert_eq!(answer.status, StatusCode::OK, "{}", answer.body);
    assert_eq!(answer.body["data"][0]["full_name"], repository["full_name"]);
}
