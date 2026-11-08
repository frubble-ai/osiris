//! Commit statuses and check runs, read from and set on GitHub and put in our terms.

#![allow(
    clippy::unwrap_used,
    clippy::indexing_slicing,
    reason = "tests: a failure here is the test failing"
)]

use super::{
    CreateCommitStatus, GetCommitStatuses, ListCheckRunsForRef, RerequestCheckRun,
    RerequestCheckSuite,
    mocks::{
        MockGithubChecksNetwork, generate_github_check_run::generate_github_check_run,
        generate_github_commit_status::generate_github_commit_status,
    },
};
use crate::{
    ProviderError,
    github::mocks::{MockGithubNetwork, github_on::github_on},
};
use rstest::rstest;
use serde_json::json;
use wiremock::MockServer;

#[tokio::test]
async fn sets_a_status_on_a_commit_with_only_the_fields_given() {
    let server = MockServer::start().await;
    let answer = generate_github_commit_status(7, "success");
    MockGithubChecksNetwork::on(&server)
        .answer_body(
            "POST",
            "/repos/octo/hello/statuses/abc123",
            json!({ "state": "success", "context": "ci/build" }),
            201,
            answer.clone(),
        )
        .await;
    let input = CreateCommitStatus {
        owner: "octo".into(),
        repo: "hello".into(),
        sha: "abc123".into(),
        state: "success".into(),
        context: Some("ci/build".into()),
        target_url: None,
        description: None,
    };

    let status = github_on(&server)
        .create_commit_status(&reqwest::Client::new(), "token", &input)
        .await
        .unwrap();

    assert_eq!(status.id, "7");
    assert_eq!(status.state, "success");
    assert_eq!(status.context, answer["context"].as_str().unwrap());
    assert_eq!(status.target_url.as_deref(), answer["target_url"].as_str());
}

#[tokio::test]
async fn leaves_out_a_blank_link_and_trims_a_padded_context_of_a_status() {
    let server = MockServer::start().await;
    MockGithubChecksNetwork::on(&server)
        .answer_body(
            "POST",
            "/repos/octo/hello/statuses/abc123",
            json!({ "state": "success", "context": "ci/build", "description": " Built " }),
            201,
            generate_github_commit_status(7, "success"),
        )
        .await;
    let input = CreateCommitStatus {
        owner: "octo".into(),
        repo: "hello".into(),
        sha: "abc123".into(),
        state: "success".into(),
        context: Some("  ci/build  ".into()),
        target_url: Some("   ".into()),
        description: Some(" Built ".into()),
    };

    let status = github_on(&server)
        .create_commit_status(&reqwest::Client::new(), "token", &input)
        .await
        .unwrap();

    assert_eq!(status.id, "7");
}

#[tokio::test]
async fn lists_a_page_of_the_statuses_on_a_branch() {
    let server = MockServer::start().await;
    let statuses = [
        generate_github_commit_status(2, "failure"),
        generate_github_commit_status(1, "pending"),
    ];
    MockGithubNetwork::on(&server)
        .answer(
            "GET",
            "/repos/octo/hello/commits/feature%2Flogin/statuses",
            &[("per_page", "2"), ("page", "1")],
            200,
            json!(statuses),
        )
        .await;
    let input = GetCommitStatuses {
        owner: "octo".into(),
        repo: "hello".into(),
        git_ref: "feature/login".into(),
        per_page: Some(2),
        page: Some(1),
    };

    let listed = github_on(&server)
        .get_commit_statuses(&reqwest::Client::new(), "token", &input)
        .await
        .unwrap();

    assert_eq!(listed.len(), 2);
    assert_eq!(listed[0].id, "2");
    assert_eq!(listed[0].state, "failure");
    assert_eq!(listed[1].context, statuses[1]["context"].as_str().unwrap());
}

#[tokio::test]
async fn lists_the_check_runs_of_a_ref_filtered_as_asked() {
    let server = MockServer::start().await;
    let runs = [
        generate_github_check_run(11, "success"),
        generate_github_check_run(12, "failure"),
    ];
    MockGithubNetwork::on(&server)
        .answer(
            "GET",
            "/repos/octo/hello/commits/main/check-runs",
            &[
                ("app_id", "15368"),
                ("check_name", "build"),
                ("filter", "all"),
                ("status", "completed"),
                ("per_page", "50"),
                ("page", "2"),
            ],
            200,
            json!({ "total_count": 2, "check_runs": runs }),
        )
        .await;
    let input = ListCheckRunsForRef {
        owner: "octo".into(),
        repo: "hello".into(),
        git_ref: "main".into(),
        app_id: Some(15368),
        check_name: Some("build".into()),
        filter: Some("all".into()),
        status: Some("completed".into()),
        per_page: Some(50),
        page: Some(2),
    };

    let listed = github_on(&server)
        .list_check_runs_for_ref(&reqwest::Client::new(), "token", &input)
        .await
        .unwrap();

    assert_eq!(listed.len(), 2);
    assert_eq!(listed[0].id, "11");
    assert_eq!(listed[1].conclusion.as_deref(), Some("failure"));
    assert_eq!(listed[0].head_sha, runs[0]["head_sha"].as_str().unwrap());
    assert_eq!(listed[0].url.as_deref(), runs[0]["html_url"].as_str());
}

#[tokio::test]
async fn leaves_out_a_blank_check_name_and_trims_a_padded_status() {
    let server = MockServer::start().await;
    MockGithubNetwork::on(&server)
        .answer(
            "GET",
            "/repos/octo/hello/commits/main/check-runs",
            &[("status", "completed")],
            200,
            json!({ "total_count": 0, "check_runs": [] }),
        )
        .await;
    let input = ListCheckRunsForRef {
        owner: "octo".into(),
        repo: "hello".into(),
        git_ref: "main".into(),
        app_id: None,
        check_name: Some("   ".into()),
        filter: None,
        status: Some(" completed ".into()),
        per_page: None,
        page: None,
    };

    let listed = github_on(&server)
        .list_check_runs_for_ref(&reqwest::Client::new(), "token", &input)
        .await
        .unwrap();

    assert!(listed.is_empty());
    let requests = server.received_requests().await.unwrap();
    assert!(
        !requests[0]
            .url
            .query_pairs()
            .any(|(name, _)| name == "check_name")
    );
}

#[rstest]
#[case::with_an_empty_object(json!({}))]
#[case::with_nothing(serde_json::Value::Null)]
#[tokio::test]
async fn acknowledges_a_check_run_rerequested(#[case] answer: serde_json::Value) {
    let server = MockServer::start().await;
    MockGithubNetwork::on(&server)
        .answer(
            "POST",
            "/repos/octo/hello/check-runs/42/rerequest",
            &[],
            201,
            answer,
        )
        .await;
    let input = RerequestCheckRun {
        owner: "octo".into(),
        repo: "hello".into(),
        check_run_id: 42,
    };

    let acknowledged = github_on(&server)
        .rerequest_check_run(&reqwest::Client::new(), "token", &input)
        .await
        .unwrap();

    assert!(acknowledged.ok);
}

#[tokio::test]
async fn acknowledges_a_check_suite_rerequested() {
    let server = MockServer::start().await;
    MockGithubNetwork::on(&server)
        .answer(
            "POST",
            "/repos/octo/hello/check-suites/9/rerequest",
            &[],
            201,
            json!({}),
        )
        .await;
    let input = RerequestCheckSuite {
        owner: "octo".into(),
        repo: "hello".into(),
        check_suite_id: 9,
    };

    let acknowledged = github_on(&server)
        .rerequest_check_suite(&reqwest::Client::new(), "token", &input)
        .await
        .unwrap();

    assert!(acknowledged.ok);
}

#[tokio::test]
async fn refuses_a_check_run_github_will_not_rerequest() {
    let server = MockServer::start().await;
    MockGithubNetwork::on(&server)
        .answer(
            "POST",
            "/repos/octo/hello/check-runs/42/rerequest",
            &[],
            403,
            json!({ "message": "Resource not accessible by integration" }),
        )
        .await;
    let input = RerequestCheckRun {
        owner: "octo".into(),
        repo: "hello".into(),
        check_run_id: 42,
    };

    let refused = github_on(&server)
        .rerequest_check_run(&reqwest::Client::new(), "token", &input)
        .await;

    assert!(matches!(refused, Err(ProviderError::Refused { .. })));
}
