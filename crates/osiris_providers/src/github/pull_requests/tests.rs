//! Pull requests, their files, commits, reviews, review comments and reviewers, read from and
//! changed on GitHub and put in our terms, with every value placed in the path percent-encoded.

#![allow(
    clippy::unwrap_used,
    clippy::indexing_slicing,
    reason = "tests: a failure here is the test failing"
)]

use super::{
    CheckPullRequestMerged, CreatePullRequest, CreatePullRequestReview,
    CreatePullRequestReviewComment, DeletePendingPullRequestReview, DeletePullRequestReviewComment,
    DiffSide, DismissPullRequestReview, GetPullRequest, GetPullRequestReview,
    ListPullRequestCommits, ListPullRequestFiles, ListPullRequestRequestedReviewers,
    ListPullRequestReviewComments, ListPullRequestReviews, ListPullRequests,
    ListPullRequestsAssociatedWithCommit, MergeMethod, MergePullRequest, PullRequestSort,
    PullRequestState, PullRequestStateFilter, RemovePullRequestReviewers,
    ReplyPullRequestReviewComment, RequestPullRequestReviewers, ReviewCommentInput,
    ReviewCommentSort, ReviewEvent, SortDirection, SubmitPullRequestReview, UpdatePullRequest,
    UpdatePullRequestBranch, UpdatePullRequestReviewComment,
    mocks::{
        MockPullRequestNetwork, generate_github_commit::generate_github_commit,
        generate_github_pull_request::generate_github_pull_request,
        generate_github_pull_request_file::generate_github_pull_request_file,
        generate_github_review::generate_github_review,
        generate_github_review_comment::generate_github_review_comment,
        generate_github_team::generate_github_team,
    },
};
use crate::{
    ProviderError,
    github::mocks::{
        MockGithubNetwork, generate_github_user::generate_github_user, generate_sha::generate_sha,
        github_on::github_on,
    },
};
use fake::{Fake, faker::lorem::en::Sentence};
use rstest::rstest;
use serde_json::{Value, json};
use wiremock::MockServer;

fn http() -> reqwest::Client {
    reqwest::Client::new()
}

fn text(value: &Value) -> &str {
    value.as_str().unwrap()
}

#[tokio::test]
async fn lists_a_page_of_pull_requests_filtered_and_sorted() {
    let server = MockServer::start().await;
    let pulls = [
        generate_github_pull_request(11, 1),
        generate_github_pull_request(12, 2),
    ];
    MockGithubNetwork::on(&server)
        .answer(
            "GET",
            "/repos/octo/hello/pulls",
            &[
                ("state", "all"),
                ("head", "octo:feature"),
                ("base", "main"),
                ("sort", "long-running"),
                ("direction", "asc"),
                ("per_page", "2"),
                ("page", "3"),
            ],
            200,
            json!(pulls),
        )
        .await;
    let input = ListPullRequests {
        owner: "octo".into(),
        repo: "hello".into(),
        state: Some(PullRequestStateFilter::All),
        head: Some("octo:feature".into()),
        base: Some("main".into()),
        sort: Some(PullRequestSort::LongRunning),
        direction: Some(SortDirection::Asc),
        per_page: Some(2),
        page: Some(3),
    };

    let listed = github_on(&server)
        .list_pull_requests(&http(), "token", &input)
        .await
        .unwrap();

    assert_eq!(listed.len(), 2);
    assert_eq!(listed[0].id, "11");
    assert_eq!(listed[1].number, 2);
    assert_eq!(listed[0].title, text(&pulls[0]["title"]));
    assert_eq!(listed[0].url, text(&pulls[0]["html_url"]));
    assert_eq!(listed[0].head.ref_name, text(&pulls[0]["head"]["ref"]));
    assert_eq!(listed[0].head.sha, text(&pulls[0]["head"]["sha"]));
    assert_eq!(listed[0].base.ref_name, "main");
    assert_eq!(
        listed[0].base.repository_full_name.as_deref(),
        pulls[0]["base"]["repo"]["full_name"].as_str()
    );
    assert_eq!(
        listed[0].user.as_ref().unwrap().login,
        text(&pulls[0]["user"]["login"])
    );
    assert!(!listed[0].merged);
}

#[tokio::test]
async fn lists_the_pull_requests_a_commit_belongs_to() {
    let server = MockServer::start().await;
    let sha = generate_sha();
    let mut merged = generate_github_pull_request(21, 7);
    merged["merged_at"] = json!("2024-01-03T00:00:00Z");
    MockGithubNetwork::on(&server)
        .answer(
            "GET",
            &format!("/repos/octo/hello/commits/{sha}/pulls"),
            &[("per_page", "10"), ("page", "1")],
            200,
            json!([merged]),
        )
        .await;
    let input = ListPullRequestsAssociatedWithCommit {
        owner: "octo".into(),
        repo: "hello".into(),
        commit_sha: sha,
        per_page: Some(10),
        page: Some(1),
    };

    let listed = github_on(&server)
        .list_pull_requests_associated_with_commit(&http(), "token", &input)
        .await
        .unwrap();

    assert_eq!(listed.len(), 1);
    assert_eq!(listed[0].number, 7);
    assert!(listed[0].merged);
    assert_eq!(listed[0].merged_at.as_deref(), Some("2024-01-03T00:00:00Z"));
}

#[tokio::test]
async fn lists_the_files_a_pull_request_changes() {
    let server = MockServer::start().await;
    let file = generate_github_pull_request_file(3, 2);
    MockGithubNetwork::on(&server)
        .answer(
            "GET",
            "/repos/octo/hello/pulls/5/files",
            &[("per_page", "50")],
            200,
            json!([file]),
        )
        .await;
    let input = ListPullRequestFiles {
        owner: "octo".into(),
        repo: "hello".into(),
        pull_number: 5,
        per_page: Some(50),
        page: None,
    };

    let files = github_on(&server)
        .list_pull_request_files(&http(), "token", &input)
        .await
        .unwrap();

    assert_eq!(files.len(), 1);
    assert_eq!(files[0].filename, text(&file["filename"]));
    assert_eq!(files[0].sha, text(&file["sha"]));
    assert_eq!(files[0].status, "modified");
    assert_eq!(
        (files[0].additions, files[0].deletions, files[0].changes),
        (3, 2, 5)
    );
    assert_eq!(files[0].patch.as_deref(), file["patch"].as_str());
}

#[tokio::test]
async fn lists_the_commits_on_a_pull_request() {
    let server = MockServer::start().await;
    let commit = generate_github_commit(77);
    MockGithubNetwork::on(&server)
        .answer(
            "GET",
            "/repos/octo/hello/pulls/5/commits",
            &[("page", "2")],
            200,
            json!([commit]),
        )
        .await;
    let input = ListPullRequestCommits {
        owner: "octo".into(),
        repo: "hello".into(),
        pull_number: 5,
        per_page: None,
        page: Some(2),
    };

    let commits = github_on(&server)
        .list_pull_request_commits(&http(), "token", &input)
        .await
        .unwrap();

    assert_eq!(commits.len(), 1);
    assert_eq!(commits[0].sha, text(&commit["sha"]));
    assert_eq!(commits[0].message, text(&commit["commit"]["message"]));
    assert_eq!(
        commits[0].author_email.as_deref(),
        commit["commit"]["author"]["email"].as_str()
    );
    assert_eq!(commits[0].author.as_ref().unwrap().id, "77");
    assert_eq!(commits[0].parents, vec![text(&commit["parents"][0]["sha"])]);
}

#[tokio::test]
async fn lists_who_a_pull_requests_review_is_requested_from() {
    let server = MockServer::start().await;
    let (user, team) = (generate_github_user(31), generate_github_team(41));
    MockGithubNetwork::on(&server)
        .answer(
            "GET",
            "/repos/octo/hello/pulls/5/requested_reviewers",
            &[],
            200,
            json!({ "users": [user], "teams": [team] }),
        )
        .await;
    let input = ListPullRequestRequestedReviewers {
        owner: "octo".into(),
        repo: "hello".into(),
        pull_number: 5,
    };

    let requested = github_on(&server)
        .list_pull_request_requested_reviewers(&http(), "token", &input)
        .await
        .unwrap();

    assert_eq!(requested.users.len(), 1);
    assert_eq!(requested.users[0].login, text(&user["login"]));
    assert_eq!(requested.teams.len(), 1);
    assert_eq!(requested.teams[0].id, "41");
    assert_eq!(requested.teams[0].slug, text(&team["slug"]));
}

#[tokio::test]
async fn lists_the_reviews_of_a_pull_request() {
    let server = MockServer::start().await;
    let reviews = [
        generate_github_review(51, "APPROVED"),
        generate_github_review(52, "CHANGES_REQUESTED"),
    ];
    MockGithubNetwork::on(&server)
        .answer(
            "GET",
            "/repos/octo/hello/pulls/5/reviews",
            &[("per_page", "2"), ("page", "1")],
            200,
            json!(reviews),
        )
        .await;
    let input = ListPullRequestReviews {
        owner: "octo".into(),
        repo: "hello".into(),
        pull_number: 5,
        per_page: Some(2),
        page: Some(1),
    };

    let listed = github_on(&server)
        .list_pull_request_reviews(&http(), "token", &input)
        .await
        .unwrap();

    assert_eq!(listed.len(), 2);
    assert_eq!(listed[0].id, "51");
    assert_eq!(listed[0].state, "APPROVED");
    assert_eq!(listed[1].state, "CHANGES_REQUESTED");
    assert_eq!(listed[0].url, text(&reviews[0]["html_url"]));
    assert_eq!(
        listed[0].commit_id.as_deref(),
        reviews[0]["commit_id"].as_str()
    );
}

#[tokio::test]
async fn lists_the_comments_on_a_pull_requests_diff_sorted_and_since_a_time() {
    let server = MockServer::start().await;
    let comments = [
        generate_github_review_comment(61, None),
        generate_github_review_comment(62, Some(61)),
    ];
    MockGithubNetwork::on(&server)
        .answer(
            "GET",
            "/repos/octo/hello/pulls/5/comments",
            &[
                ("sort", "updated"),
                ("direction", "desc"),
                ("since", "2024-01-01T00:00:00Z"),
                ("per_page", "2"),
            ],
            200,
            json!(comments),
        )
        .await;
    let input = ListPullRequestReviewComments {
        owner: "octo".into(),
        repo: "hello".into(),
        pull_number: 5,
        sort: Some(ReviewCommentSort::Updated),
        direction: Some(SortDirection::Desc),
        since: Some("2024-01-01T00:00:00Z".into()),
        per_page: Some(2),
        page: None,
    };

    let listed = github_on(&server)
        .list_pull_request_review_comments(&http(), "token", &input)
        .await
        .unwrap();

    assert_eq!(listed.len(), 2);
    assert_eq!(listed[0].id, "61");
    assert_eq!(listed[0].path, text(&comments[0]["path"]));
    assert_eq!(listed[0].body, text(&comments[0]["body"]));
    assert_eq!(listed[0].line, comments[0]["line"].as_i64());
    assert_eq!(listed[0].side.as_deref(), Some("RIGHT"));
    assert_eq!(listed[0].in_reply_to_id, None);
    assert_eq!(listed[1].in_reply_to_id.as_deref(), Some("61"));
}

#[tokio::test]
async fn creates_a_review_with_comments_on_lines_of_the_diff() {
    let server = MockServer::start().await;
    let review = generate_github_review(71, "CHANGES_REQUESTED");
    let sha = generate_sha();
    let (summary, remark): (String, String) = (Sentence(3..6).fake(), Sentence(3..6).fake());
    MockPullRequestNetwork::on(&server)
        .answer_body(
            "POST",
            "/repos/octo/hello/pulls/5/reviews",
            json!({
                "body": summary,
                "event": "REQUEST_CHANGES",
                "commit_id": sha,
                "comments": [
                    { "path": "src/main.rs", "body": remark, "line": 12, "side": "RIGHT" },
                    {
                        "path": "src/lib.rs",
                        "body": remark,
                        "line": 9,
                        "side": "LEFT",
                        "start_line": 4,
                        "start_side": "LEFT",
                    },
                ],
            }),
            200,
            review.clone(),
        )
        .await;
    let input = CreatePullRequestReview {
        owner: "octo".into(),
        repo: "hello".into(),
        pull_number: 5,
        body: Some(summary),
        event: Some(ReviewEvent::RequestChanges),
        commit_id: Some(sha),
        comments: Some(vec![
            ReviewCommentInput {
                path: "src/main.rs".into(),
                body: remark.clone(),
                line: Some(12),
                side: Some(DiffSide::Right),
                start_line: None,
                start_side: None,
            },
            ReviewCommentInput {
                path: "src/lib.rs".into(),
                body: remark.clone(),
                line: Some(9),
                side: Some(DiffSide::Left),
                start_line: Some(4),
                start_side: Some(DiffSide::Left),
            },
        ]),
    };

    let created = github_on(&server)
        .create_pull_request_review(&http(), "token", &input)
        .await
        .unwrap();

    assert_eq!(created.id, "71");
    assert_eq!(created.state, "CHANGES_REQUESTED");
    assert_eq!(created.body.as_deref(), review["body"].as_str());
}

#[tokio::test]
async fn creates_a_pending_review_when_nothing_is_given() {
    let server = MockServer::start().await;
    MockPullRequestNetwork::on(&server)
        .answer_body(
            "POST",
            "/repos/octo/hello/pulls/5/reviews",
            json!({}),
            200,
            generate_github_review(72, "PENDING"),
        )
        .await;
    let input = CreatePullRequestReview {
        owner: "octo".into(),
        repo: "hello".into(),
        pull_number: 5,
        body: None,
        event: None,
        commit_id: None,
        comments: None,
    };

    let created = github_on(&server)
        .create_pull_request_review(&http(), "token", &input)
        .await
        .unwrap();

    assert_eq!(created.state, "PENDING");
}

#[rstest]
#[case::approve(ReviewEvent::Approve, "APPROVE", "APPROVED")]
#[case::request_changes(ReviewEvent::RequestChanges, "REQUEST_CHANGES", "CHANGES_REQUESTED")]
#[case::comment(ReviewEvent::Comment, "COMMENT", "COMMENTED")]
#[tokio::test]
async fn submits_a_pending_review_with_its_verdict(
    #[case] event: ReviewEvent,
    #[case] named: &str,
    #[case] state: &str,
) {
    let server = MockServer::start().await;
    let body: String = Sentence(3..6).fake();
    MockPullRequestNetwork::on(&server)
        .answer_body(
            "POST",
            "/repos/octo/hello/pulls/5/reviews/81/events",
            json!({ "event": named, "body": body }),
            200,
            generate_github_review(81, state),
        )
        .await;
    let input = SubmitPullRequestReview {
        owner: "octo".into(),
        repo: "hello".into(),
        pull_number: 5,
        review_id: 81,
        event,
        body: Some(body),
    };

    let submitted = github_on(&server)
        .submit_pull_request_review(&http(), "token", &input)
        .await
        .unwrap();

    assert_eq!(submitted.id, "81");
    assert_eq!(submitted.state, state);
}

#[tokio::test]
async fn reads_a_review_by_id() {
    let server = MockServer::start().await;
    let review = generate_github_review(91, "COMMENTED");
    MockGithubNetwork::on(&server)
        .answer(
            "GET",
            "/repos/octo/hello/pulls/5/reviews/91",
            &[],
            200,
            review.clone(),
        )
        .await;
    let input = GetPullRequestReview {
        owner: "octo".into(),
        repo: "hello".into(),
        pull_number: 5,
        review_id: 91,
    };

    let read = github_on(&server)
        .get_pull_request_review(&http(), "token", &input)
        .await
        .unwrap();

    assert_eq!(read.id, "91");
    assert_eq!(read.user.unwrap().login, text(&review["user"]["login"]));
    assert_eq!(
        read.submitted_at.as_deref(),
        review["submitted_at"].as_str()
    );
}

#[tokio::test]
async fn dismisses_a_review_with_a_reason() {
    let server = MockServer::start().await;
    let reason: String = Sentence(3..6).fake();
    MockPullRequestNetwork::on(&server)
        .answer_body(
            "PUT",
            "/repos/octo/hello/pulls/5/reviews/92/dismissals",
            json!({ "message": reason }),
            200,
            generate_github_review(92, "DISMISSED"),
        )
        .await;
    let input = DismissPullRequestReview {
        owner: "octo".into(),
        repo: "hello".into(),
        pull_number: 5,
        review_id: 92,
        message: reason,
    };

    let dismissed = github_on(&server)
        .dismiss_pull_request_review(&http(), "token", &input)
        .await
        .unwrap();

    assert_eq!(dismissed.state, "DISMISSED");
}

#[tokio::test]
async fn deletes_a_pending_review_and_answers_it() {
    let server = MockServer::start().await;
    MockGithubNetwork::on(&server)
        .answer(
            "DELETE",
            "/repos/octo/hello/pulls/5/reviews/93",
            &[],
            200,
            generate_github_review(93, "PENDING"),
        )
        .await;
    let input = DeletePendingPullRequestReview {
        owner: "octo".into(),
        repo: "hello".into(),
        pull_number: 5,
        review_id: 93,
    };

    let deleted = github_on(&server)
        .delete_pending_pull_request_review(&http(), "token", &input)
        .await
        .unwrap();

    assert_eq!(deleted.id, "93");
    assert_eq!(deleted.state, "PENDING");
}

#[tokio::test]
async fn comments_on_a_range_of_lines_of_the_diff() {
    let server = MockServer::start().await;
    let comment = generate_github_review_comment(101, None);
    let sha = generate_sha();
    let body: String = Sentence(3..6).fake();
    MockPullRequestNetwork::on(&server)
        .answer_body(
            "POST",
            "/repos/octo/hello/pulls/5/comments",
            json!({
                "body": body,
                "commit_id": sha,
                "path": "src/main.rs",
                "line": 20,
                "side": "RIGHT",
                "start_line": 18,
                "start_side": "RIGHT",
            }),
            201,
            comment.clone(),
        )
        .await;
    let input = CreatePullRequestReviewComment {
        owner: "octo".into(),
        repo: "hello".into(),
        pull_number: 5,
        body,
        commit_id: sha,
        path: "src/main.rs".into(),
        line: Some(20),
        side: Some(DiffSide::Right),
        start_line: Some(18),
        start_side: Some(DiffSide::Right),
    };

    let created = github_on(&server)
        .create_pull_request_review_comment(&http(), "token", &input)
        .await
        .unwrap();

    assert_eq!(created.id, "101");
    assert_eq!(created.url, text(&comment["html_url"]));
    assert_eq!(created.diff_hunk, text(&comment["diff_hunk"]));
    assert_eq!(created.commit_id, text(&comment["commit_id"]));
}

#[tokio::test]
async fn replies_to_a_comment_on_the_diff() {
    let server = MockServer::start().await;
    let body: String = Sentence(3..6).fake();
    MockPullRequestNetwork::on(&server)
        .answer_body(
            "POST",
            "/repos/octo/hello/pulls/5/comments/101/replies",
            json!({ "body": body }),
            201,
            generate_github_review_comment(102, Some(101)),
        )
        .await;
    let input = ReplyPullRequestReviewComment {
        owner: "octo".into(),
        repo: "hello".into(),
        pull_number: 5,
        comment_id: 101,
        body,
    };

    let reply = github_on(&server)
        .reply_pull_request_review_comment(&http(), "token", &input)
        .await
        .unwrap();

    assert_eq!(reply.id, "102");
    assert_eq!(reply.in_reply_to_id.as_deref(), Some("101"));
}

#[tokio::test]
async fn replaces_the_text_of_a_comment_on_the_diff() {
    let server = MockServer::start().await;
    let body: String = Sentence(3..6).fake();
    let mut comment = generate_github_review_comment(103, None);
    comment["body"] = json!(body);
    MockPullRequestNetwork::on(&server)
        .answer_body(
            "PATCH",
            "/repos/octo/hello/pulls/comments/103",
            json!({ "body": body }),
            200,
            comment,
        )
        .await;
    let input = UpdatePullRequestReviewComment {
        owner: "octo".into(),
        repo: "hello".into(),
        comment_id: 103,
        body: body.clone(),
    };

    let updated = github_on(&server)
        .update_pull_request_review_comment(&http(), "token", &input)
        .await
        .unwrap();

    assert_eq!(updated.id, "103");
    assert_eq!(updated.body, body);
}

#[tokio::test]
async fn deletes_a_comment_on_the_diff_and_acknowledges_it() {
    let server = MockServer::start().await;
    MockGithubNetwork::on(&server)
        .answer(
            "DELETE",
            "/repos/octo/hello/pulls/comments/104",
            &[],
            204,
            Value::Null,
        )
        .await;
    let input = DeletePullRequestReviewComment {
        owner: "octo".into(),
        repo: "hello".into(),
        comment_id: 104,
    };

    let deleted = github_on(&server)
        .delete_pull_request_review_comment(&http(), "token", &input)
        .await
        .unwrap();

    assert!(deleted.ok);
}

#[tokio::test]
async fn reads_a_pull_request_by_number() {
    let server = MockServer::start().await;
    let mut pull = generate_github_pull_request(111, 9);
    pull["merged"] = json!(true);
    pull["draft"] = json!(true);
    MockGithubNetwork::on(&server)
        .answer("GET", "/repos/octo/hello/pulls/9", &[], 200, pull.clone())
        .await;
    let input = GetPullRequest {
        owner: "octo".into(),
        repo: "hello".into(),
        pull_number: 9,
    };

    let read = github_on(&server)
        .get_pull_request(&http(), "token", &input)
        .await
        .unwrap();

    assert_eq!(read.id, "111");
    assert_eq!(read.number, 9);
    assert_eq!(read.state, "open");
    assert_eq!(read.body.as_deref(), pull["body"].as_str());
    assert!(read.merged);
    assert!(read.draft);
    assert_eq!(read.created_at, text(&pull["created_at"]));
}

#[tokio::test]
async fn opens_a_pull_request_from_one_branch_into_another() {
    let server = MockServer::start().await;
    let (title, body): (String, String) = (Sentence(3..6).fake(), Sentence(3..6).fake());
    let mut pull = generate_github_pull_request(121, 10);
    pull["title"] = json!(title);
    MockPullRequestNetwork::on(&server)
        .answer_body(
            "POST",
            "/repos/octo/hello/pulls",
            json!({
                "title": title,
                "head": "octo:feature",
                "base": "main",
                "body": body,
                "draft": true,
                "maintainer_can_modify": false,
            }),
            201,
            pull,
        )
        .await;
    let input = CreatePullRequest {
        owner: "octo".into(),
        repo: "hello".into(),
        title: title.clone(),
        head: "octo:feature".into(),
        base: "main".into(),
        body: Some(body),
        draft: Some(true),
        maintainer_can_modify: Some(false),
    };

    let created = github_on(&server)
        .create_pull_request(&http(), "token", &input)
        .await
        .unwrap();

    assert_eq!(created.id, "121");
    assert_eq!(created.number, 10);
    assert_eq!(created.title, title);
}

#[tokio::test]
async fn changes_only_what_is_given_of_a_pull_request() {
    let server = MockServer::start().await;
    let mut pull = generate_github_pull_request(131, 11);
    pull["state"] = json!("closed");
    pull["closed_at"] = json!("2024-01-04T00:00:00Z");
    MockPullRequestNetwork::on(&server)
        .answer_body(
            "PATCH",
            "/repos/octo/hello/pulls/11",
            json!({ "state": "closed", "base": "release" }),
            200,
            pull,
        )
        .await;
    let input = UpdatePullRequest {
        owner: "octo".into(),
        repo: "hello".into(),
        pull_number: 11,
        title: None,
        body: None,
        state: Some(PullRequestState::Closed),
        base: Some("release".into()),
        maintainer_can_modify: None,
    };

    let updated = github_on(&server)
        .update_pull_request(&http(), "token", &input)
        .await
        .unwrap();

    assert_eq!(updated.state, "closed");
    assert_eq!(updated.closed_at.as_deref(), Some("2024-01-04T00:00:00Z"));
}

#[rstest]
#[case::at_an_expected_head(Some(generate_sha()))]
#[case::wherever_it_is(None)]
#[tokio::test]
async fn brings_a_pull_requests_branch_up_to_date(#[case] expected_head_sha: Option<String>) {
    let server = MockServer::start().await;
    let message: String = Sentence(3..6).fake();
    let expected = match &expected_head_sha {
        Some(sha) => json!({ "expected_head_sha": sha }),
        None => json!({}),
    };
    MockPullRequestNetwork::on(&server)
        .answer_body(
            "PUT",
            "/repos/octo/hello/pulls/12/update-branch",
            expected,
            202,
            json!({ "message": message, "url": "https://api.github.com/repos/octo/hello/pulls/12" }),
        )
        .await;
    let input = UpdatePullRequestBranch {
        owner: "octo".into(),
        repo: "hello".into(),
        pull_number: 12,
        expected_head_sha,
    };

    let update = github_on(&server)
        .update_pull_request_branch(&http(), "token", &input)
        .await
        .unwrap();

    assert_eq!(update.message, message);
    assert_eq!(
        update.url,
        "https://api.github.com/repos/octo/hello/pulls/12"
    );
}

/// A pull request whose review is requested from `user` and `team`.
fn pull_request_requesting(user: &Value, team: &Value) -> Value {
    let mut pull = generate_github_pull_request(141, 13);
    pull["requested_reviewers"] = json!([user]);
    pull["requested_teams"] = json!([team]);
    pull
}

#[tokio::test]
async fn requests_a_review_from_users_and_teams() {
    let server = MockServer::start().await;
    let (user, team) = (generate_github_user(151), generate_github_team(161));
    MockPullRequestNetwork::on(&server)
        .answer_body(
            "POST",
            "/repos/octo/hello/pulls/13/requested_reviewers",
            json!({ "reviewers": [user["login"]], "team_reviewers": [team["slug"]] }),
            201,
            pull_request_requesting(&user, &team),
        )
        .await;
    let input = RequestPullRequestReviewers {
        owner: "octo".into(),
        repo: "hello".into(),
        pull_number: 13,
        reviewers: Some(vec![text(&user["login"]).into()]),
        team_reviewers: Some(vec![text(&team["slug"]).into()]),
    };

    let requested = github_on(&server)
        .request_pull_request_reviewers(&http(), "token", &input)
        .await
        .unwrap();

    assert_eq!(requested.pull_request.id, "141");
    assert_eq!(requested.pull_request.number, 13);
    assert_eq!(requested.requested_reviewers.len(), 1);
    assert_eq!(requested.requested_reviewers[0].id, "151");
    assert_eq!(requested.requested_teams.len(), 1);
    assert_eq!(requested.requested_teams[0].slug, text(&team["slug"]));
}

#[tokio::test]
async fn removes_users_from_those_a_review_is_requested_from() {
    let server = MockServer::start().await;
    let user = generate_github_user(152);
    MockPullRequestNetwork::on(&server)
        .answer_body(
            "DELETE",
            "/repos/octo/hello/pulls/13/requested_reviewers",
            json!({ "reviewers": [user["login"]] }),
            200,
            generate_github_pull_request(142, 13),
        )
        .await;
    let input = RemovePullRequestReviewers {
        owner: "octo".into(),
        repo: "hello".into(),
        pull_number: 13,
        reviewers: Some(vec![text(&user["login"]).into()]),
        team_reviewers: None,
    };

    let remaining = github_on(&server)
        .remove_pull_request_reviewers(&http(), "token", &input)
        .await
        .unwrap();

    assert_eq!(remaining.pull_request.id, "142");
    assert!(remaining.requested_reviewers.is_empty());
    assert!(remaining.requested_teams.is_empty());
}

#[rstest]
#[case::merge(MergeMethod::Merge, "merge")]
#[case::squash(MergeMethod::Squash, "squash")]
#[case::rebase(MergeMethod::Rebase, "rebase")]
#[tokio::test]
async fn merges_a_pull_request_by_the_method_asked(
    #[case] method: MergeMethod,
    #[case] named: &str,
) {
    let server = MockServer::start().await;
    let (head, merge) = (generate_sha(), generate_sha());
    let title: String = Sentence(3..6).fake();
    MockPullRequestNetwork::on(&server)
        .answer_body(
            "PUT",
            "/repos/octo/hello/pulls/14/merge",
            json!({ "commit_title": title, "sha": head, "merge_method": named }),
            200,
            json!({ "sha": merge, "merged": true, "message": "Pull Request successfully merged" }),
        )
        .await;
    let input = MergePullRequest {
        owner: "octo".into(),
        repo: "hello".into(),
        pull_number: 14,
        commit_title: Some(title),
        commit_message: None,
        sha: Some(head),
        merge_method: Some(method),
    };

    let merged = github_on(&server)
        .merge_pull_request(&http(), "token", &input)
        .await
        .unwrap();

    assert!(merged.merged);
    assert_eq!(merged.sha, Some(merge));
    assert_eq!(merged.message, "Pull Request successfully merged");
}

#[rstest]
#[case::merged(204, true)]
#[case::not_merged(404, false)]
#[tokio::test]
async fn tells_whether_a_pull_request_has_been_merged(#[case] status: u16, #[case] merged: bool) {
    let server = MockServer::start().await;
    let answer = if status == 404 {
        json!({ "message": "Not Found" })
    } else {
        Value::Null
    };
    MockGithubNetwork::on(&server)
        .answer(
            "GET",
            "/repos/octo/hello/pulls/15/merge",
            &[],
            status,
            answer,
        )
        .await;
    let input = CheckPullRequestMerged {
        owner: "octo".into(),
        repo: "hello".into(),
        pull_number: 15,
    };

    let check = github_on(&server)
        .check_pull_request_merged(&http(), "token", &input)
        .await
        .unwrap();

    assert_eq!(check.merged, merged);
}

#[tokio::test]
async fn passes_on_any_other_refusal_of_the_merge_check() {
    let server = MockServer::start().await;
    MockGithubNetwork::on(&server)
        .answer(
            "GET",
            "/repos/octo/hello/pulls/15/merge",
            &[],
            403,
            json!({ "message": "Resource not accessible by integration" }),
        )
        .await;
    let input = CheckPullRequestMerged {
        owner: "octo".into(),
        repo: "hello".into(),
        pull_number: 15,
    };

    let refused = github_on(&server)
        .check_pull_request_merged(&http(), "token", &input)
        .await;

    assert!(matches!(refused, Err(ProviderError::Refused { .. })));
}

#[tokio::test]
async fn encodes_an_owner_a_repository_and_a_commit_into_one_segment_each() {
    let server = MockServer::start().await;
    MockGithubNetwork::on(&server)
        .answer(
            "GET",
            "/repos/my%20org/hello%2Fworld/commits/abc%2F..%3F/pulls",
            &[],
            200,
            json!([]),
        )
        .await;
    let input = ListPullRequestsAssociatedWithCommit {
        owner: "my org".into(),
        repo: "hello/world".into(),
        commit_sha: "abc/..?".into(),
        per_page: None,
        page: None,
    };

    let listed = github_on(&server)
        .list_pull_requests_associated_with_commit(&http(), "token", &input)
        .await
        .unwrap();

    assert!(listed.is_empty());
}

#[tokio::test]
async fn encodes_an_owner_and_a_repository_in_a_comment_path() {
    let server = MockServer::start().await;
    MockGithubNetwork::on(&server)
        .answer(
            "DELETE",
            "/repos/my%20org/hello%2Fworld/pulls/comments/7",
            &[],
            204,
            Value::Null,
        )
        .await;
    let input = DeletePullRequestReviewComment {
        owner: "my org".into(),
        repo: "hello/world".into(),
        comment_id: 7,
    };

    let deleted = github_on(&server)
        .delete_pull_request_review_comment(&http(), "token", &input)
        .await
        .unwrap();

    assert!(deleted.ok);
}
