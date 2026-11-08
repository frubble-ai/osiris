//! Activity, read from and written to GitHub and put in our terms: the event feeds, stars and
//! watchers, with every value placed in the path percent-encoded.

#![allow(
    clippy::unwrap_used,
    clippy::indexing_slicing,
    reason = "tests: a failure here is the test failing"
)]

use super::{
    CheckRepositoryStarred, ListAuthenticatedUserEvents, ListAuthenticatedUserReceivedEvents,
    ListMyStarredRepositories, ListPublicEvents, ListRepositoryEvents, ListRepositoryStargazers,
    ListRepositoryWatchers, ListUserPublicEvents, ListUserReceivedPublicEvents, StarRepository,
    UnstarRepository, mocks::generate_github_event::generate_github_event,
};
use crate::{
    ProviderError,
    github::mocks::{
        MockGithubNetwork, generate_github::generate_github,
        generate_github_repository::generate_github_repository,
        generate_github_user::generate_github_user,
    },
};
use rstest::rstest;
use serde_json::{Value, json};
use wiremock::MockServer;

#[tokio::test]
async fn lists_a_page_of_the_public_feed() {
    let server = MockServer::start().await;
    let events = [
        generate_github_event(1, "WatchEvent"),
        generate_github_event(2, "PushEvent"),
    ];
    MockGithubNetwork::on(&server)
        .answer(
            "GET",
            "/events",
            &[("per_page", "2"), ("page", "4")],
            200,
            json!(events),
        )
        .await;
    let input = ListPublicEvents {
        per_page: Some(2),
        page: Some(4),
    };

    let listed = generate_github(&server)
        .list_public_events(&reqwest::Client::new(), "token", &input)
        .await
        .unwrap();

    assert_eq!(listed.len(), 2);
    assert_eq!(listed[0].id, "1");
    assert_eq!(listed[0].event_type, "WatchEvent");
    assert_eq!(
        listed[0].actor.as_ref().unwrap().login,
        events[0]["actor"]["login"].as_str().unwrap()
    );
    let repository = listed[0].repository.as_ref().unwrap();
    assert_eq!(repository.id, "2001");
    assert_eq!(repository.name, events[0]["repo"]["name"].as_str().unwrap());
    assert_eq!(listed[0].payload, events[0]["payload"]);
}

#[rstest]
#[case::public("public", "/users/octo%20cat/events/public")]
#[case::received_public("received_public", "/users/octo%20cat/received_events/public")]
#[case::own("own", "/users/octo%20cat/events")]
#[case::own_received("own_received", "/users/octo%20cat/received_events")]
#[tokio::test]
async fn lists_a_page_of_a_users_feed(#[case] feed: &str, #[case] route: &str) {
    let server = MockServer::start().await;
    MockGithubNetwork::on(&server)
        .answer(
            "GET",
            route,
            &[("page", "2")],
            200,
            json!([generate_github_event(3, "IssuesEvent")]),
        )
        .await;
    let github = generate_github(&server);
    let http = reqwest::Client::new();
    let username = String::from("octo cat");

    let listed = match feed {
        "public" => {
            let input = ListUserPublicEvents {
                username,
                per_page: None,
                page: Some(2),
            };
            github.list_user_public_events(&http, "token", &input).await
        }
        "received_public" => {
            let input = ListUserReceivedPublicEvents {
                username,
                per_page: None,
                page: Some(2),
            };
            github
                .list_user_received_public_events(&http, "token", &input)
                .await
        }
        "own" => {
            let input = ListAuthenticatedUserEvents {
                username,
                per_page: None,
                page: Some(2),
            };
            github
                .list_authenticated_user_events(&http, "token", &input)
                .await
        }
        _ => {
            let input = ListAuthenticatedUserReceivedEvents {
                username,
                per_page: None,
                page: Some(2),
            };
            github
                .list_authenticated_user_received_events(&http, "token", &input)
                .await
        }
    }
    .unwrap();

    assert_eq!(listed.len(), 1);
    assert_eq!(listed[0].event_type, "IssuesEvent");
}

#[tokio::test]
async fn lists_a_page_of_the_events_of_a_repository() {
    let server = MockServer::start().await;
    MockGithubNetwork::on(&server)
        .answer(
            "GET",
            "/repos/octo/hello/events",
            &[("per_page", "10")],
            200,
            json!([generate_github_event(4, "ForkEvent")]),
        )
        .await;
    let input = ListRepositoryEvents {
        owner: "octo".into(),
        repo: "hello".into(),
        per_page: Some(10),
        page: None,
    };

    let listed = generate_github(&server)
        .list_repository_events(&reqwest::Client::new(), "token", &input)
        .await
        .unwrap();

    assert_eq!(listed[0].id, "4");
    assert!(listed[0].public);
}

#[rstest]
#[case::star("PUT")]
#[case::unstar("DELETE")]
#[tokio::test]
async fn stars_and_unstars_a_repository_and_acknowledges_it(#[case] verb: &str) {
    let server = MockServer::start().await;
    MockGithubNetwork::on(&server)
        .answer(verb, "/user/starred/octo/hello", &[], 204, Value::Null)
        .await;
    let github = generate_github(&server);
    let http = reqwest::Client::new();

    let acknowledged = if verb == "PUT" {
        let input = StarRepository {
            owner: "octo".into(),
            repo: "hello".into(),
        };
        github.star_repository(&http, "token", &input).await
    } else {
        let input = UnstarRepository {
            owner: "octo".into(),
            repo: "hello".into(),
        };
        github.unstar_repository(&http, "token", &input).await
    };

    assert!(acknowledged.unwrap().ok);
}

#[rstest]
#[case::starred(204, true)]
#[case::not_starred(404, false)]
#[tokio::test]
async fn checks_whether_a_repository_is_starred(#[case] status: u16, #[case] starred: bool) {
    let server = MockServer::start().await;
    let body = if status == 404 {
        json!({ "message": "Not Found" })
    } else {
        Value::Null
    };
    MockGithubNetwork::on(&server)
        .answer("GET", "/user/starred/octo/hello", &[], status, body)
        .await;
    let input = CheckRepositoryStarred {
        owner: "octo".into(),
        repo: "hello".into(),
    };

    let check = generate_github(&server)
        .check_repository_starred(&reqwest::Client::new(), "token", &input)
        .await
        .unwrap();

    assert_eq!(check.starred, starred);
}

#[rstest]
#[case::forbidden(403)]
#[case::rejected_token(401)]
#[case::outage(503)]
#[tokio::test]
async fn a_star_check_github_refuses_otherwise_is_an_error(#[case] status: u16) {
    let server = MockServer::start().await;
    MockGithubNetwork::on(&server)
        .answer(
            "GET",
            "/user/starred/octo/hello",
            &[],
            status,
            json!({ "message": "no" }),
        )
        .await;
    let input = CheckRepositoryStarred {
        owner: "octo".into(),
        repo: "hello".into(),
    };

    let error = generate_github(&server)
        .check_repository_starred(&reqwest::Client::new(), "token", &input)
        .await
        .unwrap_err();

    let expected = match status {
        401 => matches!(error, ProviderError::Unauthorized { .. }),
        403 => matches!(error, ProviderError::Refused { .. }),
        _ => matches!(error, ProviderError::Unavailable { .. }),
    };
    assert!(expected, "{error:?}");
}

#[rstest]
#[case::stargazers("/repos/octo/hello/stargazers")]
#[case::watchers("/repos/octo/hello/subscribers")]
#[tokio::test]
async fn lists_a_page_of_the_stargazers_or_watchers_of_a_repository(#[case] route: &str) {
    let server = MockServer::start().await;
    let users = [generate_github_user(5), generate_github_user(6)];
    MockGithubNetwork::on(&server)
        .answer(
            "GET",
            route,
            &[("per_page", "2"), ("page", "1")],
            200,
            json!(users),
        )
        .await;
    let github = generate_github(&server);
    let http = reqwest::Client::new();

    let listed = if route.ends_with("stargazers") {
        let input = ListRepositoryStargazers {
            owner: "octo".into(),
            repo: "hello".into(),
            per_page: Some(2),
            page: Some(1),
        };
        github
            .list_repository_stargazers(&http, "token", &input)
            .await
    } else {
        let input = ListRepositoryWatchers {
            owner: "octo".into(),
            repo: "hello".into(),
            per_page: Some(2),
            page: Some(1),
        };
        github
            .list_repository_watchers(&http, "token", &input)
            .await
    }
    .unwrap();

    assert_eq!(listed.len(), 2);
    assert_eq!(listed[1].id, "6");
    assert_eq!(listed[1].login, users[1]["login"].as_str().unwrap());
    assert_eq!(listed[1].account_type, "User");
}

#[tokio::test]
async fn lists_the_repositories_the_user_starred_sorted_as_asked() {
    let server = MockServer::start().await;
    let repositories = [generate_github_repository(7)];
    MockGithubNetwork::on(&server)
        .answer(
            "GET",
            "/user/starred",
            &[
                ("sort", "updated"),
                ("direction", "asc"),
                ("per_page", "30"),
            ],
            200,
            json!(repositories),
        )
        .await;
    let input = ListMyStarredRepositories {
        sort: Some("updated".into()),
        direction: Some("asc".into()),
        per_page: Some(30),
        page: None,
    };

    let listed = generate_github(&server)
        .list_my_starred_repositories(&reqwest::Client::new(), "token", &input)
        .await
        .unwrap();

    assert_eq!(listed.len(), 1);
    assert_eq!(listed[0].id, "7");
    assert_eq!(
        listed[0].full_name,
        repositories[0]["full_name"].as_str().unwrap()
    );
}

#[tokio::test]
async fn sends_the_starred_sort_trimmed_and_leaves_out_a_blank_direction() {
    let server = MockServer::start().await;
    MockGithubNetwork::on(&server)
        .answer_filtered(
            "/user/starred",
            &[("sort", "created")],
            &["direction"],
            json!([]),
        )
        .await;
    let input = ListMyStarredRepositories {
        sort: Some(" created  ".into()),
        direction: Some("   ".into()),
        per_page: None,
        page: None,
    };

    let listed = generate_github(&server)
        .list_my_starred_repositories(&reqwest::Client::new(), "token", &input)
        .await
        .unwrap();

    assert!(listed.is_empty());
}
