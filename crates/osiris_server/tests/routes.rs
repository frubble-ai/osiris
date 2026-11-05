//! Every route the release lists, through the router: auth configs, the validation errors, the
//! connection routes, test events and each action.

#![allow(
    clippy::unwrap_used,
    clippy::indexing_slicing,
    reason = "tests: a failure here is the test failing"
)]

#[path = "support/app.rs"]
mod support;

use axum::http::StatusCode;
use osiris_domain::EventType;
use serde_json::json;
use sqlx::PgPool;
use support::{ADMIN_KEY, API_KEY, app};
use wiremock::{
    Mock, ResponseTemplate,
    matchers::{header, method, path, path_regex, query_param},
};

#[sqlx::test(migrator = "osiris_storage::MIGRATOR")]
async fn manages_auth_configs_without_ever_returning_the_secret(pool: PgPool) {
    let app = app(pool).await;
    let id = app.github_config().await;

    let read = app
        .send(
            "GET",
            &format!("/v1/auth-configs/{id}"),
            Some(ADMIN_KEY),
            None,
            &[],
        )
        .await;
    assert_eq!(read.status, StatusCode::OK);
    assert_eq!(read.body["provider"], "github");
    assert!(read.body.get("client_secret").is_none());

    let updated = app
        .send(
            "PATCH",
            &format!("/v1/auth-configs/{id}"),
            Some(ADMIN_KEY),
            Some(json!({ "client_id": "Iv1.other", "scopes": ["repo"] })),
            &[],
        )
        .await;
    assert_eq!(updated.status, StatusCode::OK, "{}", updated.body);
    assert_eq!(updated.body["client_id"], "Iv1.other");
    assert_eq!(updated.body["scopes"], json!(["repo"]));

    let listed = app
        .send("GET", "/v1/auth-configs", Some(ADMIN_KEY), None, &[])
        .await;
    assert_eq!(listed.body["data"].as_array().unwrap().len(), 1);

    let deleted = app
        .send(
            "DELETE",
            &format!("/v1/auth-configs/{id}"),
            Some(ADMIN_KEY),
            None,
            &[],
        )
        .await;
    assert_eq!(deleted.status, StatusCode::OK);
    assert_eq!(deleted.body["deleted"], true);
    let gone = app
        .send(
            "GET",
            &format!("/v1/auth-configs/{id}"),
            Some(ADMIN_KEY),
            None,
            &[],
        )
        .await;
    assert_eq!(gone.status, StatusCode::NOT_FOUND);
    assert_eq!(gone.body["error"]["code"], "OS_01");
}

#[sqlx::test(migrator = "osiris_storage::MIGRATOR")]
async fn refuses_to_delete_an_auth_config_a_connection_uses(pool: PgPool) {
    let app = app(pool).await;
    let config = app.github_config().await;
    app.start_connect(&config).await;
    let answer = app
        .send(
            "DELETE",
            &format!("/v1/auth-configs/{config}"),
            Some(ADMIN_KEY),
            None,
            &[],
        )
        .await;
    assert_eq!(answer.status, StatusCode::CONFLICT, "{}", answer.body);
    assert_eq!(answer.body["error"]["code"], "OS_03");
}

#[sqlx::test(migrator = "osiris_storage::MIGRATOR")]
async fn answers_invalid_requests_with_os_02(pool: PgPool) {
    let app = app(pool).await;
    let config = app.github_config().await;
    let invalid = [
        (
            "/v1/auth-configs",
            ADMIN_KEY,
            json!({ "provider": "nope", "client_id": "c", "client_secret": "s" }),
        ),
        (
            "/v1/auth-configs",
            ADMIN_KEY,
            json!({ "provider": "github", "client_id": "c", "client_secret": "" }),
        ),
        (
            "/v1/connected-accounts",
            API_KEY,
            json!({ "auth_config_id": config, "user_id": "org_1", "callback_url": "ftp://app.test/cb" }),
        ),
        (
            "/v1/connected-accounts",
            API_KEY,
            json!({ "auth_config_id": config, "user_id": " ", "callback_url": "http://app.test/cb" }),
        ),
    ];
    for (route, key, body) in invalid {
        let answer = app
            .send("POST", route, Some(key), Some(body.clone()), &[])
            .await;
        assert_eq!(
            answer.status,
            StatusCode::BAD_REQUEST,
            "{body}: {}",
            answer.body
        );
        assert_eq!(answer.body["error"]["code"], "OS_02", "{body}");
        assert!(answer.body["error"]["message"].is_string());
    }
}

#[sqlx::test(migrator = "osiris_storage::MIGRATOR")]
async fn deletes_a_connection_whose_grant_the_provider_no_longer_has(pool: PgPool) {
    let app = app(pool).await;
    let id = app.connected(28800).await;
    Mock::given(method("DELETE"))
        .and(path_regex("^/applications/.*/grant$"))
        .respond_with(ResponseTemplate::new(404))
        .mount(&app.provider)
        .await;
    let answer = app
        .send(
            "DELETE",
            &format!("/v1/connected-accounts/{id}"),
            Some(API_KEY),
            None,
            &[],
        )
        .await;
    assert_eq!(answer.status, StatusCode::OK, "{}", answer.body);
    let gone = app
        .send(
            "GET",
            &format!("/v1/connected-accounts/{id}"),
            Some(API_KEY),
            None,
            &[],
        )
        .await;
    assert_eq!(gone.status, StatusCode::NOT_FOUND);
}

#[sqlx::test(migrator = "osiris_storage::MIGRATOR")]
async fn keeps_a_connection_when_the_provider_cannot_end_its_grant(pool: PgPool) {
    let app = app(pool).await;
    let id = app.connected(28800).await;
    Mock::given(method("DELETE"))
        .and(path_regex("^/applications/.*/grant$"))
        .respond_with(ResponseTemplate::new(503))
        .mount(&app.provider)
        .await;
    let answer = app
        .send(
            "DELETE",
            &format!("/v1/connected-accounts/{id}"),
            Some(API_KEY),
            None,
            &[],
        )
        .await;
    assert_eq!(answer.status, StatusCode::BAD_GATEWAY, "{}", answer.body);
    assert_eq!(answer.body["error"]["code"], "OS_05");
    assert!(
        !answer.body["error"]["message"]
            .as_str()
            .unwrap()
            .contains("503"),
        "the provider's answer stays in the logs"
    );
    assert_eq!(app.account(&id).await["status"], "active");
    let refresh = osiris_tasks::names::refresh(&id, 1);
    assert!(
        !app.tasks.cancelled().contains(&refresh) && app.tasks.get(&refresh).is_some(),
        "its refresh stays scheduled"
    );
}

#[sqlx::test(migrator = "osiris_storage::MIGRATOR")]
async fn triggers_each_event_type_and_leaves_the_connection_as_it_is(pool: PgPool) {
    let app = app(pool).await;
    let id = app.connected(28800).await;
    let before = app.events().len();
    for (event_type, expected) in [
        ("connection.active", EventType::ConnectionActive),
        ("connection.expired", EventType::ConnectionExpired),
        ("connection.revoked", EventType::ConnectionRevoked),
    ] {
        let answer = app
            .send(
                "POST",
                "/v1/events/trigger",
                Some(ADMIN_KEY),
                Some(json!({ "type": event_type, "connected_account_id": id })),
                &[],
            )
            .await;
        assert_eq!(answer.status, StatusCode::CREATED, "{}", answer.body);
        assert_eq!(app.events().last().unwrap().event_type, expected);
    }
    assert_eq!(app.events().len(), before + 3);
    assert_eq!(app.account(&id).await["status"], "active");
}

#[sqlx::test(migrator = "osiris_storage::MIGRATOR")]
async fn refuses_an_unknown_event_type_or_connection(pool: PgPool) {
    let app = app(pool).await;
    let id = app.connected(28800).await;
    let unknown_type = app
        .send(
            "POST",
            "/v1/events/trigger",
            Some(ADMIN_KEY),
            Some(json!({ "type": "connection.nope", "connected_account_id": id })),
            &[],
        )
        .await;
    assert_eq!(unknown_type.status, StatusCode::BAD_REQUEST);
    let unknown_connection = app
        .send(
            "POST",
            "/v1/events/trigger",
            Some(ADMIN_KEY),
            Some(json!({ "type": "connection.active", "connected_account_id": "ca_missing" })),
            &[],
        )
        .await;
    assert_eq!(unknown_connection.status, StatusCode::NOT_FOUND);
    let with_api_key = app
        .send(
            "POST",
            "/v1/events/trigger",
            Some(API_KEY),
            Some(json!({ "type": "connection.active", "connected_account_id": id })),
            &[],
        )
        .await;
    assert_eq!(with_api_key.status, StatusCode::UNAUTHORIZED);
}

#[sqlx::test(migrator = "osiris_storage::MIGRATOR")]
async fn lists_github_repositories_through_the_router(pool: PgPool) {
    let app = app(pool).await;
    let id = app.connected(28800).await;
    Mock::given(method("GET")).and(path("/user/installations"))
        .respond_with(ResponseTemplate::new(200).set_body_json(json!({ "total_count": 1, "installations": [{ "id": 1, "account": { "login": "a", "type": "User" }, "repository_selection": "all" }] })))
        .mount(&app.provider).await;
    Mock::given(method("GET")).and(path("/user/installations/1/repositories"))
        .respond_with(ResponseTemplate::new(200).set_body_json(json!({ "total_count": 1, "repositories": [{ "id": 9, "full_name": "a/r", "private": true, "html_url": "https://github.com/a/r", "description": null }] })))
        .mount(&app.provider).await;
    let answer = app
        .send(
            "POST",
            &format!("/v1/connected-accounts/{id}/actions/github.list_repositories"),
            Some(API_KEY),
            None,
            &[],
        )
        .await;
    assert_eq!(answer.status, StatusCode::OK, "{}", answer.body);
    assert_eq!(answer.body["data"][0]["full_name"], "a/r");
}

#[sqlx::test(migrator = "osiris_storage::MIGRATOR")]
async fn lists_linear_teams_through_the_router(pool: PgPool) {
    let app = app(pool).await;
    let id = app.connected_to("linear", "/oauth/token").await;
    Mock::given(method("POST")).and(path("/graphql")).and(header("authorization", "Bearer linear-token"))
        .respond_with(ResponseTemplate::new(200).set_body_json(json!({ "data": { "teams": { "nodes": [{ "id": "t1", "name": "Core" }], "pageInfo": { "hasNextPage": false, "endCursor": null } } } })))
        .mount(&app.provider).await;
    let answer = app
        .send(
            "POST",
            &format!("/v1/connected-accounts/{id}/actions/linear.list_teams"),
            Some(API_KEY),
            None,
            &[],
        )
        .await;
    assert_eq!(answer.status, StatusCode::OK, "{}", answer.body);
    assert_eq!(answer.body["data"][0]["name"], "Core");
}

#[sqlx::test(migrator = "osiris_storage::MIGRATOR")]
async fn lists_slack_channels_through_the_router(pool: PgPool) {
    let app = app(pool).await;
    let id = app.connected_to("slack", "/api/oauth.v2.user.access").await;
    Mock::given(method("GET")).and(path("/api/conversations.list")).and(query_param("cursor", ""))
        .respond_with(ResponseTemplate::new(200).set_body_json(json!({ "ok": true, "channels": [{ "id": "C1", "name": "general", "is_private": false, "num_members": 3 }], "response_metadata": { "next_cursor": "" } })))
        .mount(&app.provider).await;
    let answer = app
        .send(
            "POST",
            &format!("/v1/connected-accounts/{id}/actions/slack.list_channels"),
            Some(API_KEY),
            None,
            &[],
        )
        .await;
    assert_eq!(answer.status, StatusCode::OK, "{}", answer.body);
    assert_eq!(answer.body["data"][0]["name"], "general");
}

#[sqlx::test(migrator = "osiris_storage::MIGRATOR")]
async fn answers_health_with_the_commit_and_503_once_the_database_is_down(pool: PgPool) {
    let app = app(pool.clone()).await;
    let up = app.send("GET", "/health", None, None, &[]).await;
    assert_eq!(up.status, StatusCode::OK);
    assert_eq!(up.body["database"], true);
    assert!(up.body["commit"].is_string());
    assert_eq!(app.send_to_worker_get("/health").await, StatusCode::OK);

    pool.close().await;
    let down = app.send("GET", "/health", None, None, &[]).await;
    assert_eq!(down.status, StatusCode::SERVICE_UNAVAILABLE);
    assert_eq!(down.body["database"], false);
    assert_eq!(
        app.send_to_worker_get("/health").await,
        StatusCode::SERVICE_UNAVAILABLE
    );
}

#[sqlx::test(migrator = "osiris_storage::MIGRATOR")]
async fn answers_the_worker_s_health_with_503_once_it_is_draining(pool: PgPool) {
    let app = app(pool).await;
    assert_eq!(app.send_to_worker_get("/health").await, StatusCode::OK);
    app.draining
        .store(true, std::sync::atomic::Ordering::SeqCst);
    assert_eq!(
        app.send_to_worker_get("/health").await,
        StatusCode::SERVICE_UNAVAILABLE
    );
}

#[sqlx::test(migrator = "osiris_storage::MIGRATOR")]
async fn never_returns_the_provider_s_refusal_text_to_the_client(pool: PgPool) {
    let app = app(pool).await;
    let id = app.connected(28800).await;
    Mock::given(method("GET"))
        .and(path("/user/installations"))
        .respond_with(ResponseTemplate::new(403).set_body_string("secret internal detail"))
        .mount(&app.provider)
        .await;
    let answer = app
        .send(
            "POST",
            &format!("/v1/connected-accounts/{id}/actions/github.list_installations"),
            Some(API_KEY),
            None,
            &[],
        )
        .await;
    assert_eq!(
        answer.status,
        StatusCode::FAILED_DEPENDENCY,
        "{}",
        answer.body
    );
    assert_eq!(answer.body["error"]["code"], "OS_04");
    assert!(!answer.body.to_string().contains("secret internal detail"));
}

#[sqlx::test(migrator = "osiris_storage::MIGRATOR")]
async fn ends_the_grant_at_the_provider_when_deleting_a_disabled_connection(pool: PgPool) {
    let app = app(pool).await;
    let id = app.connected(28800).await;
    Mock::given(method("POST"))
        .and(path("/login/oauth/access_token"))
        .and(wiremock::matchers::body_string_contains(
            "grant_type=refresh_token",
        ))
        .respond_with(ResponseTemplate::new(200).set_body_json(
            json!({ "access_token": "ghu_2", "refresh_token": "ghr_2", "expires_in": 28800 }),
        ))
        .mount(&app.provider)
        .await;
    let task = app
        .tasks
        .take(&osiris_tasks::names::refresh(&id, 1))
        .unwrap();
    app.tasks.fail(true);
    app.deliver(&task).await;
    app.tasks.fail(false);
    assert_eq!(app.account(&id).await["status"], "disabled");
    Mock::given(method("DELETE"))
        .and(path_regex("^/applications/.*/grant$"))
        .respond_with(ResponseTemplate::new(204))
        .expect(1)
        .mount(&app.provider)
        .await;

    let answer = app
        .send(
            "DELETE",
            &format!("/v1/connected-accounts/{id}"),
            Some(API_KEY),
            None,
            &[],
        )
        .await;
    assert_eq!(answer.status, StatusCode::OK, "{}", answer.body);
}

#[sqlx::test(migrator = "osiris_storage::MIGRATOR")]
async fn stores_a_fixed_reason_when_the_consent_comes_back_with_an_unknown_error(pool: PgPool) {
    let app = app(pool).await;
    let config = app.github_config().await;
    let (id, state) = app.start_connect(&config).await;
    app.send(
        "GET",
        &format!("/oauth/callback?state={state}&error=%3Cscript%3Ealert(1)%3C%2Fscript%3E"),
        None,
        None,
        &[],
    )
    .await;
    let account = app.account(&id).await;
    assert_eq!(account["status"], "failed");
    assert_eq!(account["last_error"], "the provider refused the consent");
}
