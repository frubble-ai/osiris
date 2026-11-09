//! GitHub Actions' workflows, runs, jobs, logs and artifacts, read from and acted on at GitHub and
//! put in our terms.

#![allow(
    clippy::unwrap_used,
    clippy::indexing_slicing,
    reason = "tests: a failure here is the test failing"
)]

use super::{
    CancelWorkflowRun, DisableWorkflow, DispatchWorkflow, DownloadWorkflowArtifact, EnableWorkflow,
    GetWorkflow, GetWorkflowJobLogs, GetWorkflowRun, ListRepositoryWorkflows,
    ListWorkflowRunArtifacts, ListWorkflowRunJobs, ListWorkflowRuns, RerunFailedJobs,
    RerunWorkflow,
    mocks::{
        MockGithubWorkflowsNetwork, generate_github_artifact::generate_github_artifact,
        generate_github_workflow::generate_github_workflow,
        generate_github_workflow_job::generate_github_workflow_job,
        generate_github_workflow_run::generate_github_workflow_run,
    },
};
use crate::github::github_id::GithubId;
use crate::{
    ListLimits, ProviderError, ProviderUrls,
    github::{
        Github,
        mocks::{
            MockGithubNetwork,
            generate_github::generate_github,
            generate_github_names::{
                generate_github_login, generate_github_repo, generate_github_token,
            },
        },
    },
};
use fake::{Fake, faker::lorem::en::Paragraph};
use rstest::rstest;
use serde_json::json;
use std::collections::BTreeMap;
use wiremock::MockServer;

#[tokio::test]
async fn lists_a_page_of_the_workflows_of_a_repository() {
    let server = MockServer::start().await;
    let owner = generate_github_login();
    let repo = generate_github_repo();
    let workflows = [generate_github_workflow(1), generate_github_workflow(2)];
    MockGithubNetwork::on(&server)
        .answer(
            "GET",
            &format!("/repos/{owner}/{repo}/actions/workflows"),
            &[("per_page", "2"), ("page", "1")],
            200,
            json!({ "total_count": 2, "workflows": workflows }),
        )
        .await;
    let input = ListRepositoryWorkflows {
        owner: owner.clone(),
        repo: repo.clone(),
        per_page: Some(2),
        page: Some(1),
    };

    let listed = generate_github(&server)
        .list_repository_workflows(&reqwest::Client::new(), &generate_github_token(), &input)
        .await
        .unwrap();

    assert_eq!(listed.len(), 2);
    assert_eq!(listed[1].id, "2");
    assert_eq!(listed[0].path, workflows[0]["path"].as_str().unwrap());
    assert_eq!(listed[0].url, workflows[0]["html_url"].as_str().unwrap());
}

#[rstest]
#[case::by_id("161335", "161335")]
#[case::by_file_name("ci.yml", "ci.yml")]
#[case::by_a_name_that_would_leave_the_path("../ci.yml", "..%2Fci.yml")]
#[tokio::test]
async fn reads_a_workflow_by_id_or_file_name(#[case] workflow_id: &str, #[case] segment: &str) {
    let server = MockServer::start().await;
    let owner = generate_github_login();
    let repo = generate_github_repo();
    MockGithubNetwork::on(&server)
        .answer(
            "GET",
            &format!("/repos/{owner}/{repo}/actions/workflows/{segment}"),
            &[],
            200,
            generate_github_workflow(161_335),
        )
        .await;
    let input = GetWorkflow {
        owner: owner.clone(),
        repo: repo.clone(),
        workflow_id: workflow_id.into(),
    };

    let workflow = generate_github(&server)
        .get_workflow(&reqwest::Client::new(), &generate_github_token(), &input)
        .await
        .unwrap();

    assert_eq!(workflow.id, "161335");
    assert_eq!(workflow.state, "active");
}

#[rstest]
#[case::with_inputs(
    Some(BTreeMap::from([("environment".to_owned(), "staging".to_owned())])),
    json!({ "ref": "main", "inputs": { "environment": "staging" } })
)]
#[case::without_inputs(None, json!({ "ref": "main" }))]
#[tokio::test]
async fn dispatches_a_workflow_on_a_ref(
    #[case] inputs: Option<BTreeMap<String, String>>,
    #[case] body: serde_json::Value,
) {
    let server = MockServer::start().await;
    let owner = generate_github_login();
    let repo = generate_github_repo();
    MockGithubWorkflowsNetwork::on(&server)
        .answer_body(
            "POST",
            &format!("/repos/{owner}/{repo}/actions/workflows/deploy.yml/dispatches"),
            body,
            204,
            serde_json::Value::Null,
        )
        .await;
    let input = DispatchWorkflow {
        owner: owner.clone(),
        repo: repo.clone(),
        workflow_id: "deploy.yml".into(),
        git_ref: "main".into(),
        inputs,
    };

    let acknowledged = generate_github(&server)
        .dispatch_workflow(&reqwest::Client::new(), &generate_github_token(), &input)
        .await
        .unwrap();

    assert!(acknowledged.ok);
}

#[tokio::test]
async fn enables_a_workflow() {
    let server = MockServer::start().await;
    let owner = generate_github_login();
    let repo = generate_github_repo();
    MockGithubNetwork::on(&server)
        .answer(
            "PUT",
            &format!("/repos/{owner}/{repo}/actions/workflows/ci.yml/enable"),
            &[],
            204,
            serde_json::Value::Null,
        )
        .await;
    let input = EnableWorkflow {
        owner: owner.clone(),
        repo: repo.clone(),
        workflow_id: "ci.yml".into(),
    };

    let acknowledged = generate_github(&server)
        .enable_workflow(&reqwest::Client::new(), &generate_github_token(), &input)
        .await
        .unwrap();

    assert!(acknowledged.ok);
}

#[tokio::test]
async fn disables_a_workflow() {
    let server = MockServer::start().await;
    let owner = generate_github_login();
    let repo = generate_github_repo();
    MockGithubNetwork::on(&server)
        .answer(
            "PUT",
            &format!("/repos/{owner}/{repo}/actions/workflows/161335/disable"),
            &[],
            204,
            serde_json::Value::Null,
        )
        .await;
    let input = DisableWorkflow {
        owner: owner.clone(),
        repo: repo.clone(),
        workflow_id: "161335".into(),
    };

    let acknowledged = generate_github(&server)
        .disable_workflow(&reqwest::Client::new(), &generate_github_token(), &input)
        .await
        .unwrap();

    assert!(acknowledged.ok);
}

#[tokio::test]
async fn lists_workflow_runs_filtered_as_asked() {
    let server = MockServer::start().await;
    let owner = generate_github_login();
    let repo = generate_github_repo();
    let actor = generate_github_login();
    let runs = [
        generate_github_workflow_run(30, 161_335),
        generate_github_workflow_run(29, 161_335),
    ];
    MockGithubNetwork::on(&server)
        .answer(
            "GET",
            &format!("/repos/{owner}/{repo}/actions/runs"),
            &[
                ("actor", actor.as_str()),
                ("branch", "main"),
                ("created", ">=2024-01-01"),
                ("check_suite_id", "77"),
                ("event", "push"),
                ("head_sha", "abc123"),
                ("status", "completed"),
                ("exclude_pull_requests", "true"),
                ("per_page", "10"),
                ("page", "1"),
            ],
            200,
            json!({ "total_count": 2, "workflow_runs": runs }),
        )
        .await;
    let input = ListWorkflowRuns {
        owner: owner.clone(),
        repo: repo.clone(),
        actor: Some(actor.clone()),
        branch: Some("main".into()),
        created: Some(">=2024-01-01".into()),
        check_suite_id: Some(GithubId(77)),
        event: Some("push".into()),
        head_sha: Some("abc123".into()),
        status: Some("completed".into()),
        exclude_pull_requests: Some(true),
        per_page: Some(10),
        page: Some(1),
    };

    let listed = generate_github(&server)
        .list_workflow_runs(&reqwest::Client::new(), &generate_github_token(), &input)
        .await
        .unwrap();

    assert_eq!(listed.len(), 2);
    assert_eq!(listed[0].id, "30");
    assert_eq!(listed[0].workflow_id, "161335");
    assert_eq!(listed[1].head_sha, runs[1]["head_sha"].as_str().unwrap());
}

#[tokio::test]
async fn leaves_out_a_blank_actor_and_trims_a_padded_branch_of_a_run_list() {
    let server = MockServer::start().await;
    let owner = generate_github_login();
    let repo = generate_github_repo();
    MockGithubNetwork::on(&server)
        .answer(
            "GET",
            &format!("/repos/{owner}/{repo}/actions/runs"),
            &[("branch", "main")],
            200,
            json!({ "total_count": 0, "workflow_runs": [] }),
        )
        .await;
    let input = ListWorkflowRuns {
        owner: owner.clone(),
        repo: repo.clone(),
        actor: Some("   ".into()),
        branch: Some("  main  ".into()),
        created: None,
        check_suite_id: None,
        event: None,
        head_sha: None,
        status: None,
        exclude_pull_requests: None,
        per_page: None,
        page: None,
    };

    let listed = generate_github(&server)
        .list_workflow_runs(&reqwest::Client::new(), &generate_github_token(), &input)
        .await
        .unwrap();

    assert!(listed.is_empty());
    let requests = server.received_requests().await.unwrap();
    assert!(
        !requests[0]
            .url
            .query_pairs()
            .any(|(name, _)| name == "actor")
    );
}

#[tokio::test]
async fn reads_a_workflow_run_by_id() {
    let server = MockServer::start().await;
    let owner = generate_github_login();
    let repo = generate_github_repo();
    let answer = generate_github_workflow_run(30, 161_335);
    MockGithubNetwork::on(&server)
        .answer(
            "GET",
            &format!("/repos/{owner}/{repo}/actions/runs/30"),
            &[],
            200,
            answer.clone(),
        )
        .await;
    let input = GetWorkflowRun {
        owner: owner.clone(),
        repo: repo.clone(),
        run_id: GithubId(30),
    };

    let run = generate_github(&server)
        .get_workflow_run(&reqwest::Client::new(), &generate_github_token(), &input)
        .await
        .unwrap();

    assert_eq!(run.id, "30");
    assert_eq!(run.display_title, answer["display_title"].as_str().unwrap());
    assert_eq!(run.url, answer["html_url"].as_str().unwrap());
}

#[tokio::test]
async fn lists_the_jobs_of_a_run_with_their_steps() {
    let server = MockServer::start().await;
    let owner = generate_github_login();
    let repo = generate_github_repo();
    let jobs = [generate_github_workflow_job(400, 30)];
    MockGithubNetwork::on(&server)
        .answer(
            "GET",
            &format!("/repos/{owner}/{repo}/actions/runs/30/jobs"),
            &[("filter", "all"), ("per_page", "5")],
            200,
            json!({ "total_count": 1, "jobs": jobs }),
        )
        .await;
    let input = ListWorkflowRunJobs {
        owner: owner.clone(),
        repo: repo.clone(),
        run_id: GithubId(30),
        filter: Some("all".into()),
        per_page: Some(5),
        page: None,
    };

    let listed = generate_github(&server)
        .list_workflow_run_jobs(&reqwest::Client::new(), &generate_github_token(), &input)
        .await
        .unwrap();

    assert_eq!(listed.len(), 1);
    assert_eq!(listed[0].id, "400");
    assert_eq!(listed[0].run_id, "30");
    assert_eq!(listed[0].steps.len(), 1);
    assert_eq!(
        listed[0].steps[0].name,
        jobs[0]["steps"][0]["name"].as_str().unwrap()
    );
}

#[tokio::test]
async fn reads_the_logs_of_a_job_from_where_github_redirects() {
    let server = MockServer::start().await;
    let owner = generate_github_login();
    let repo = generate_github_repo();
    let logs: String = Paragraph(3..6).fake();
    MockGithubWorkflowsNetwork::on(&server)
        .answer_redirect(
            &format!("/repos/{owner}/{repo}/actions/jobs/400/logs"),
            "/storage/logs/400.txt",
            200,
            &logs,
        )
        .await;
    let input = GetWorkflowJobLogs {
        owner: owner.clone(),
        repo: repo.clone(),
        job_id: GithubId(400),
    };

    let read = generate_github(&server)
        .get_workflow_job_logs(&reqwest::Client::new(), &generate_github_token(), &input)
        .await
        .unwrap();

    assert_eq!(read.logs, logs);
    assert!(!read.truncated);
}

#[tokio::test]
async fn keeps_only_the_end_of_a_log_longer_than_osiris_answers() {
    let server = MockServer::start().await;
    let owner = generate_github_login();
    let repo = generate_github_repo();
    let logs = format!("{}THE-END", "x".repeat(100));
    MockGithubWorkflowsNetwork::on(&server)
        .answer_redirect(
            &format!("/repos/{owner}/{repo}/actions/jobs/400/logs"),
            "/storage/logs/400.txt",
            200,
            &logs,
        )
        .await;
    let base = url::Url::parse(&server.uri()).unwrap();
    let github = Github::new(
        &ProviderUrls {
            oauth_base: base.clone(),
            api_base: base,
        },
        ListLimits {
            concurrency: 1,
            max_items: 10,
            max_log_bytes: 10,
            max_scan_pages: 10,
        },
    )
    .unwrap();
    let input = GetWorkflowJobLogs {
        owner: owner.clone(),
        repo: repo.clone(),
        job_id: GithubId(400),
    };

    let read = github
        .get_workflow_job_logs(&reqwest::Client::new(), &generate_github_token(), &input)
        .await
        .unwrap();

    assert_eq!(read.logs, "xxxTHE-END");
    assert!(read.truncated);
}

#[tokio::test]
async fn refuses_the_logs_of_a_job_github_no_longer_keeps() {
    let server = MockServer::start().await;
    let owner = generate_github_login();
    let repo = generate_github_repo();
    MockGithubNetwork::on(&server)
        .answer(
            "GET",
            &format!("/repos/{owner}/{repo}/actions/jobs/400/logs"),
            &[],
            410,
            json!({ "message": "Gone" }),
        )
        .await;
    let input = GetWorkflowJobLogs {
        owner: owner.clone(),
        repo: repo.clone(),
        job_id: GithubId(400),
    };

    let refused = generate_github(&server)
        .get_workflow_job_logs(&reqwest::Client::new(), &generate_github_token(), &input)
        .await;

    assert!(matches!(refused, Err(ProviderError::Refused { .. })));
}

#[rstest]
#[case::with_debug_logging(Some(true), json!({ "enable_debug_logging": true }))]
#[case::as_github_defaults(None, json!({}))]
#[tokio::test]
async fn reruns_a_whole_workflow_run(
    #[case] enable_debug_logging: Option<bool>,
    #[case] body: serde_json::Value,
) {
    let server = MockServer::start().await;
    let owner = generate_github_login();
    let repo = generate_github_repo();
    MockGithubWorkflowsNetwork::on(&server)
        .answer_body(
            "POST",
            &format!("/repos/{owner}/{repo}/actions/runs/30/rerun"),
            body,
            201,
            json!({}),
        )
        .await;
    let input = RerunWorkflow {
        owner: owner.clone(),
        repo: repo.clone(),
        run_id: GithubId(30),
        enable_debug_logging,
    };

    let acknowledged = generate_github(&server)
        .rerun_workflow(&reqwest::Client::new(), &generate_github_token(), &input)
        .await
        .unwrap();

    assert!(acknowledged.ok);
}

#[tokio::test]
async fn reruns_only_the_failed_jobs_of_a_run() {
    let server = MockServer::start().await;
    let owner = generate_github_login();
    let repo = generate_github_repo();
    MockGithubWorkflowsNetwork::on(&server)
        .answer_body(
            "POST",
            &format!("/repos/{owner}/{repo}/actions/runs/30/rerun-failed-jobs"),
            json!({ "enable_debug_logging": false }),
            201,
            json!({}),
        )
        .await;
    let input = RerunFailedJobs {
        owner: owner.clone(),
        repo: repo.clone(),
        run_id: GithubId(30),
        enable_debug_logging: Some(false),
    };

    let acknowledged = generate_github(&server)
        .rerun_failed_jobs(&reqwest::Client::new(), &generate_github_token(), &input)
        .await
        .unwrap();

    assert!(acknowledged.ok);
}

#[tokio::test]
async fn cancels_a_workflow_run() {
    let server = MockServer::start().await;
    let owner = generate_github_login();
    let repo = generate_github_repo();
    MockGithubNetwork::on(&server)
        .answer(
            "POST",
            &format!("/repos/{owner}/{repo}/actions/runs/30/cancel"),
            &[],
            202,
            json!({}),
        )
        .await;
    let input = CancelWorkflowRun {
        owner: owner.clone(),
        repo: repo.clone(),
        run_id: GithubId(30),
    };

    let acknowledged = generate_github(&server)
        .cancel_workflow_run(&reqwest::Client::new(), &generate_github_token(), &input)
        .await
        .unwrap();

    assert!(acknowledged.ok);
}

#[tokio::test]
async fn lists_the_artifacts_of_a_run_by_name() {
    let server = MockServer::start().await;
    let owner = generate_github_login();
    let repo = generate_github_repo();
    let artifacts = [generate_github_artifact(500)];
    MockGithubNetwork::on(&server)
        .answer(
            "GET",
            &format!("/repos/{owner}/{repo}/actions/runs/30/artifacts"),
            &[("name", "coverage"), ("page", "1")],
            200,
            json!({ "total_count": 1, "artifacts": artifacts }),
        )
        .await;
    let input = ListWorkflowRunArtifacts {
        owner: owner.clone(),
        repo: repo.clone(),
        run_id: GithubId(30),
        name: Some("coverage".into()),
        per_page: None,
        page: Some(1),
    };

    let listed = generate_github(&server)
        .list_workflow_run_artifacts(&reqwest::Client::new(), &generate_github_token(), &input)
        .await
        .unwrap();

    assert_eq!(listed.len(), 1);
    assert_eq!(listed[0].id, "500");
    assert_eq!(
        listed[0].size_in_bytes,
        artifacts[0]["size_in_bytes"].as_i64().unwrap()
    );
    assert!(!listed[0].expired);
}

#[tokio::test]
async fn answers_the_signed_url_github_redirects_an_artifact_to() {
    let server = MockServer::start().await;
    let owner = generate_github_login();
    let repo = generate_github_repo();
    MockGithubWorkflowsNetwork::on(&server)
        .answer_redirect(
            &format!("/repos/{owner}/{repo}/actions/artifacts/500/zip"),
            "/storage/artifacts/500.zip",
            200,
            "PK",
        )
        .await;
    let input = DownloadWorkflowArtifact {
        owner: owner.clone(),
        repo: repo.clone(),
        artifact_id: GithubId(500),
    };

    let download = generate_github(&server)
        .download_workflow_artifact(&reqwest::Client::new(), &generate_github_token(), &input)
        .await
        .unwrap();

    assert_eq!(
        download.url,
        format!("{}/storage/artifacts/500.zip", server.uri())
    );
}

#[tokio::test]
async fn refuses_an_artifact_that_has_expired() {
    let server = MockServer::start().await;
    let owner = generate_github_login();
    let repo = generate_github_repo();
    MockGithubNetwork::on(&server)
        .answer(
            "GET",
            &format!("/repos/{owner}/{repo}/actions/artifacts/500/zip"),
            &[],
            410,
            json!({ "message": "Artifact has expired" }),
        )
        .await;
    let input = DownloadWorkflowArtifact {
        owner: owner.clone(),
        repo: repo.clone(),
        artifact_id: GithubId(500),
    };

    let refused = generate_github(&server)
        .download_workflow_artifact(&reqwest::Client::new(), &generate_github_token(), &input)
        .await;

    assert!(matches!(refused, Err(ProviderError::Refused { .. })));
}
