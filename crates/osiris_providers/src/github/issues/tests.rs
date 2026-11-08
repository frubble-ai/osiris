//! Issues, their labels, milestones, assignees, comments, events and reactions, read from and
//! written to GitHub and put in our terms, with every value placed in the path percent-encoded and
//! every body sent as GitHub names its fields.

#![allow(
    clippy::unwrap_used,
    clippy::indexing_slicing,
    reason = "tests: a failure here is the test failing"
)]

use super::{
    AddIssueAssignees, AddIssueLabels, ClearIssueLabels, CreateIssue, CreateIssueComment,
    CreateIssueCommentReaction, CreateIssueReaction, CreateLabel, CreateMilestone,
    DeleteIssueComment, DeleteLabel, DeleteMilestone, GetIssue, GetIssueComment, GetLabel,
    GetMilestone, ListAssignees, ListIssueComments, ListIssueEvents, ListIssueLabels,
    ListIssueTimelineEvents, ListMilestones, ListRepositoryIssueEvents, ListRepositoryIssues,
    ListRepositoryLabels, LockIssue, RemoveIssueAssignees, RemoveIssueLabel,
    SearchIssuesAndPullRequests, SetIssueLabels, UnlockIssue, UpdateIssue, UpdateIssueComment,
    UpdateLabel, UpdateMilestone,
    mocks::{
        MockGithubIssuesNetwork,
        generate_github_issue::generate_github_issue,
        generate_github_issue_comment::generate_github_issue_comment,
        generate_github_issue_event::generate_github_issue_event,
        generate_github_label::generate_github_label,
        generate_github_milestone::generate_github_milestone,
        generate_github_reaction::generate_github_reaction,
        generate_github_search_issue::generate_github_search_issue,
        generate_github_timeline_event::{
            generate_github_timeline_comment, generate_github_timeline_commit,
        },
    },
};
use crate::{
    ProviderError,
    github::mocks::{
        MockGithubNetwork, generate_github::generate_github,
        generate_github_user::generate_github_user,
    },
};
use fake::{Fake, faker::lorem::en::Word};
use rstest::rstest;
use serde_json::{Value, json};
use wiremock::{
    Mock, MockServer, ResponseTemplate,
    matchers::{method, path, query_param, query_param_is_missing},
};

fn http() -> reqwest::Client {
    reqwest::Client::new()
}

fn word() -> String {
    Word().fake()
}

fn text(answer: &Value, field: &str) -> String {
    answer[field].as_str().unwrap().to_owned()
}

// Issues.

#[tokio::test]
async fn lists_a_page_of_issues_without_its_pull_requests_and_counts_the_whole_page() {
    let server = MockServer::start().await;
    let page = [
        generate_github_issue(11, 1, false),
        generate_github_issue(12, 2, true),
        generate_github_issue(13, 3, false),
    ];
    MockGithubNetwork::on(&server)
        .answer(
            "GET",
            "/repos/octo/hello/issues",
            &[
                ("state", "all"),
                ("labels", "bug,help wanted"),
                ("sort", "updated"),
                ("direction", "asc"),
                ("since", "2026-01-01T00:00:00Z"),
                ("per_page", "3"),
                ("page", "2"),
            ],
            200,
            json!(page),
        )
        .await;
    let input = ListRepositoryIssues {
        owner: "octo".into(),
        repo: "hello".into(),
        state: Some("all".into()),
        labels: Some(vec!["bug".into(), "help wanted".into()]),
        sort: Some("updated".into()),
        direction: Some("asc".into()),
        since: Some("2026-01-01T00:00:00Z".into()),
        per_page: Some(3),
        page: Some(2),
    };

    let listed = generate_github(&server)
        .list_repository_issues(&http(), "token", &input)
        .await
        .unwrap();

    assert_eq!(listed.fetched, 3);
    assert_eq!(listed.issues.len(), 2);
    assert_eq!(listed.issues[0].id, "11");
    assert_eq!(listed.issues[1].id, "13");
    assert!(listed.issues.iter().all(|issue| !issue.is_pull_request));
}

#[tokio::test]
async fn asks_for_github_s_default_page_size_when_none_is_given() {
    let server = MockServer::start().await;
    MockGithubNetwork::on(&server)
        .answer(
            "GET",
            "/repos/octo/hello/issues",
            &[("per_page", "30")],
            200,
            json!([generate_github_issue(21, 4, true)]),
        )
        .await;
    let input = ListRepositoryIssues {
        owner: "octo".into(),
        repo: "hello".into(),
        state: None,
        labels: None,
        sort: None,
        direction: None,
        since: None,
        per_page: None,
        page: None,
    };

    let listed = generate_github(&server)
        .list_repository_issues(&http(), "token", &input)
        .await
        .unwrap();

    assert_eq!(listed.fetched, 1);
    assert!(listed.issues.is_empty());
}

#[tokio::test]
async fn sends_the_issue_filters_trimmed_and_leaves_out_the_blank_ones() {
    let server = MockServer::start().await;
    Mock::given(method("GET"))
        .and(path("/repos/octo/hello/issues"))
        .and(query_param("state", "closed"))
        .and(query_param("since", "2026-11-01T00:00:00Z"))
        .and(query_param_is_missing("sort"))
        .and(query_param_is_missing("direction"))
        .respond_with(ResponseTemplate::new(200).set_body_json(json!([])))
        .expect(1)
        .mount(&server)
        .await;
    let input = ListRepositoryIssues {
        owner: "octo".into(),
        repo: "hello".into(),
        state: Some("  closed ".into()),
        labels: None,
        sort: Some("   ".into()),
        direction: Some(String::new()),
        since: Some(" 2026-11-01T00:00:00Z ".into()),
        per_page: None,
        page: None,
    };

    let listed = generate_github(&server)
        .list_repository_issues(&http(), "token", &input)
        .await
        .unwrap();

    assert_eq!(listed.fetched, 0);
}

#[tokio::test]
async fn creates_an_issue_with_only_the_fields_given() {
    let server = MockServer::start().await;
    let answer = generate_github_issue(31, 7, false);
    let title = word();
    MockGithubIssuesNetwork::on(&server)
        .answer_body(
            "POST",
            "/repos/octo/hello/issues",
            json!({ "title": title, "labels": ["bug"], "milestone": 2 }),
            201,
            Some(answer.clone()),
        )
        .await;
    let input = CreateIssue {
        owner: "octo".into(),
        repo: "hello".into(),
        title,
        body: None,
        assignees: None,
        labels: Some(vec!["bug".into()]),
        milestone: Some(2),
    };

    let issue = generate_github(&server)
        .create_issue(&http(), "token", &input)
        .await
        .unwrap();

    assert_eq!(issue.id, "31");
    assert_eq!(issue.number, 7);
    assert_eq!(issue.url, text(&answer, "html_url"));
    assert_eq!(issue.user.unwrap().id, "32");
    assert_eq!(issue.assignees[0].id, "33");
    assert_eq!(issue.labels[0].id, "34");
    assert_eq!(issue.milestone.unwrap().id, "35");
}

#[rstest]
#[case::plain("octo", "hello", "/repos/octo/hello/issues/9")]
#[case::with_reserved_characters("oc to", "hel/lo", "/repos/oc%20to/hel%2Flo/issues/9")]
#[tokio::test]
async fn reads_an_issue_with_its_owner_and_repo_encoded(
    #[case] owner: &str,
    #[case] repo: &str,
    #[case] route: &str,
) {
    let server = MockServer::start().await;
    let answer = generate_github_issue(41, 9, false);
    MockGithubNetwork::on(&server)
        .answer("GET", route, &[], 200, answer.clone())
        .await;
    let input = GetIssue {
        owner: owner.into(),
        repo: repo.into(),
        issue_number: 9,
    };

    let issue = generate_github(&server)
        .get_issue(&http(), "token", &input)
        .await
        .unwrap();

    assert_eq!(issue.id, "41");
    assert_eq!(issue.title, text(&answer, "title"));
    assert!(!issue.is_pull_request);
}

#[tokio::test]
async fn reads_a_pull_request_answered_as_an_issue_as_one() {
    let server = MockServer::start().await;
    MockGithubNetwork::on(&server)
        .answer(
            "GET",
            "/repos/octo/hello/issues/5",
            &[],
            200,
            generate_github_issue(42, 5, true),
        )
        .await;
    let input = GetIssue {
        owner: "octo".into(),
        repo: "hello".into(),
        issue_number: 5,
    };

    let issue = generate_github(&server)
        .get_issue(&http(), "token", &input)
        .await
        .unwrap();

    assert!(issue.is_pull_request);
}

#[tokio::test]
async fn updates_an_issue_with_only_the_fields_given() {
    let server = MockServer::start().await;
    MockGithubIssuesNetwork::on(&server)
        .answer_body(
            "PATCH",
            "/repos/octo/hello/issues/3",
            json!({ "state": "closed", "assignees": ["alice", "bob"] }),
            200,
            Some(generate_github_issue(51, 3, false)),
        )
        .await;
    let input = UpdateIssue {
        owner: "octo".into(),
        repo: "hello".into(),
        issue_number: 3,
        title: None,
        body: None,
        state: Some("closed".into()),
        assignees: Some(vec!["alice".into(), "bob".into()]),
        labels: None,
        milestone: None,
    };

    let issue = generate_github(&server)
        .update_issue(&http(), "token", &input)
        .await
        .unwrap();

    assert_eq!(issue.id, "51");
}

// Labels.

#[tokio::test]
async fn lists_a_page_of_a_repository_s_labels() {
    let server = MockServer::start().await;
    let labels = [generate_github_label(61), generate_github_label(62)];
    MockGithubNetwork::on(&server)
        .answer(
            "GET",
            "/repos/octo/hello/labels",
            &[("per_page", "2"), ("page", "1")],
            200,
            json!(labels),
        )
        .await;
    let input = ListRepositoryLabels {
        owner: "octo".into(),
        repo: "hello".into(),
        per_page: Some(2),
        page: Some(1),
    };

    let listed = generate_github(&server)
        .list_repository_labels(&http(), "token", &input)
        .await
        .unwrap();

    assert_eq!(listed.len(), 2);
    assert_eq!(listed[0].id, "61");
    assert_eq!(listed[1].name, text(&labels[1], "name"));
    assert_eq!(listed[1].color, text(&labels[1], "color"));
}

#[tokio::test]
async fn creates_a_label() {
    let server = MockServer::start().await;
    let answer = generate_github_label(71);
    MockGithubIssuesNetwork::on(&server)
        .answer_body(
            "POST",
            "/repos/octo/hello/labels",
            json!({ "name": "bug", "color": "d73a4a", "description": "Broken" }),
            201,
            Some(answer.clone()),
        )
        .await;
    let input = CreateLabel {
        owner: "octo".into(),
        repo: "hello".into(),
        name: "bug".into(),
        color: "d73a4a".into(),
        description: Some("Broken".into()),
    };

    let label = generate_github(&server)
        .create_label(&http(), "token", &input)
        .await
        .unwrap();

    assert_eq!(label.id, "71");
    assert_eq!(label.name, text(&answer, "name"));
}

#[tokio::test]
async fn lists_a_page_of_an_issue_s_labels() {
    let server = MockServer::start().await;
    MockGithubNetwork::on(&server)
        .answer(
            "GET",
            "/repos/octo/hello/issues/4/labels",
            &[("per_page", "10")],
            200,
            json!([generate_github_label(81)]),
        )
        .await;
    let input = ListIssueLabels {
        owner: "octo".into(),
        repo: "hello".into(),
        issue_number: 4,
        per_page: Some(10),
        page: None,
    };

    let listed = generate_github(&server)
        .list_issue_labels(&http(), "token", &input)
        .await
        .unwrap();

    assert_eq!(listed[0].id, "81");
}

#[tokio::test]
async fn adds_labels_to_an_issue_and_answers_every_label_it_has() {
    let server = MockServer::start().await;
    MockGithubIssuesNetwork::on(&server)
        .answer_body(
            "POST",
            "/repos/octo/hello/issues/4/labels",
            json!({ "labels": ["bug", "urgent"] }),
            200,
            Some(json!([
                generate_github_label(91),
                generate_github_label(92),
                generate_github_label(93),
            ])),
        )
        .await;
    let input = AddIssueLabels {
        owner: "octo".into(),
        repo: "hello".into(),
        issue_number: 4,
        labels: vec!["bug".into(), "urgent".into()],
    };

    let labels = generate_github(&server)
        .add_issue_labels(&http(), "token", &input)
        .await
        .unwrap();

    assert_eq!(labels.len(), 3);
    assert_eq!(labels[2].id, "93");
}

#[rstest]
#[case::some_labels(vec!["bug".to_owned()])]
#[case::no_label(Vec::new())]
#[tokio::test]
async fn replaces_an_issue_s_labels(#[case] labels: Vec<String>) {
    let server = MockServer::start().await;
    MockGithubIssuesNetwork::on(&server)
        .answer_body(
            "PUT",
            "/repos/octo/hello/issues/4/labels",
            json!({ "labels": labels }),
            200,
            Some(json!([generate_github_label(101)])),
        )
        .await;
    let input = SetIssueLabels {
        owner: "octo".into(),
        repo: "hello".into(),
        issue_number: 4,
        labels,
    };

    let set = generate_github(&server)
        .set_issue_labels(&http(), "token", &input)
        .await
        .unwrap();

    assert_eq!(set[0].id, "101");
}

#[tokio::test]
async fn removes_one_label_named_with_a_space_from_an_issue() {
    let server = MockServer::start().await;
    MockGithubNetwork::on(&server)
        .answer(
            "DELETE",
            "/repos/octo/hello/issues/4/labels/help%20wanted",
            &[],
            200,
            json!([generate_github_label(111)]),
        )
        .await;
    let input = RemoveIssueLabel {
        owner: "octo".into(),
        repo: "hello".into(),
        issue_number: 4,
        label: "help wanted".into(),
    };

    let kept = generate_github(&server)
        .remove_issue_label(&http(), "token", &input)
        .await
        .unwrap();

    assert_eq!(kept[0].id, "111");
}

#[tokio::test]
async fn clears_an_issue_s_labels() {
    let server = MockServer::start().await;
    MockGithubIssuesNetwork::on(&server)
        .answer_empty("DELETE", "/repos/octo/hello/issues/4/labels", 204)
        .await;
    let input = ClearIssueLabels {
        owner: "octo".into(),
        repo: "hello".into(),
        issue_number: 4,
    };

    let done = generate_github(&server)
        .clear_issue_labels(&http(), "token", &input)
        .await
        .unwrap();

    assert!(done.ok);
}

#[tokio::test]
async fn reads_a_label_by_a_name_with_reserved_characters() {
    let server = MockServer::start().await;
    let answer = generate_github_label(121);
    MockGithubNetwork::on(&server)
        .answer(
            "GET",
            "/repos/octo/hello/labels/type%3A%20bug%2Fcrash",
            &[],
            200,
            answer.clone(),
        )
        .await;
    let input = GetLabel {
        owner: "octo".into(),
        repo: "hello".into(),
        name: "type: bug/crash".into(),
    };

    let label = generate_github(&server)
        .get_label(&http(), "token", &input)
        .await
        .unwrap();

    assert_eq!(label.id, "121");
    assert_eq!(label.color, text(&answer, "color"));
}

#[tokio::test]
async fn updates_a_label_with_only_the_fields_given() {
    let server = MockServer::start().await;
    MockGithubIssuesNetwork::on(&server)
        .answer_body(
            "PATCH",
            "/repos/octo/hello/labels/bug",
            json!({ "new_name": "defect", "color": "A1b2C3" }),
            200,
            Some(generate_github_label(131)),
        )
        .await;
    let input = UpdateLabel {
        owner: "octo".into(),
        repo: "hello".into(),
        name: "bug".into(),
        new_name: Some("defect".into()),
        color: Some("A1b2C3".into()),
        description: None,
    };

    let label = generate_github(&server)
        .update_label(&http(), "token", &input)
        .await
        .unwrap();

    assert_eq!(label.id, "131");
}

#[rstest]
#[case::with_a_hash("#a1b2c3")]
#[case::too_short("abc")]
#[case::not_hex("zzzzzz")]
#[tokio::test]
async fn refuses_a_label_color_that_is_not_six_hex_digits_without_asking_github(
    #[case] color: &str,
) {
    let server = MockServer::start().await;
    MockGithubIssuesNetwork::on(&server).expect_nothing().await;
    let input = UpdateLabel {
        owner: "octo".into(),
        repo: "hello".into(),
        name: "bug".into(),
        new_name: None,
        color: Some(color.into()),
        description: None,
    };

    let error = generate_github(&server)
        .update_label(&http(), "token", &input)
        .await
        .unwrap_err();

    assert!(matches!(error, ProviderError::Refused { .. }));
}

#[tokio::test]
async fn deletes_a_label() {
    let server = MockServer::start().await;
    MockGithubIssuesNetwork::on(&server)
        .answer_empty("DELETE", "/repos/octo/hello/labels/won%27t%20fix", 204)
        .await;
    let input = DeleteLabel {
        owner: "octo".into(),
        repo: "hello".into(),
        name: "won't fix".into(),
    };

    let done = generate_github(&server)
        .delete_label(&http(), "token", &input)
        .await
        .unwrap();

    assert!(done.ok);
}

// Assignees.

#[tokio::test]
async fn assigns_people_to_an_issue() {
    let server = MockServer::start().await;
    MockGithubIssuesNetwork::on(&server)
        .answer_body(
            "POST",
            "/repos/octo/hello/issues/6/assignees",
            json!({ "assignees": ["alice"] }),
            201,
            Some(generate_github_issue(141, 6, false)),
        )
        .await;
    let input = AddIssueAssignees {
        owner: "octo".into(),
        repo: "hello".into(),
        issue_number: 6,
        assignees: vec!["alice".into()],
    };

    let issue = generate_github(&server)
        .add_issue_assignees(&http(), "token", &input)
        .await
        .unwrap();

    assert_eq!(issue.id, "141");
    assert_eq!(issue.assignees[0].id, "143");
}

#[tokio::test]
async fn unassigns_people_from_an_issue() {
    let server = MockServer::start().await;
    MockGithubIssuesNetwork::on(&server)
        .answer_body(
            "DELETE",
            "/repos/octo/hello/issues/6/assignees",
            json!({ "assignees": ["alice", "bob"] }),
            200,
            Some(generate_github_issue(151, 6, false)),
        )
        .await;
    let input = RemoveIssueAssignees {
        owner: "octo".into(),
        repo: "hello".into(),
        issue_number: 6,
        assignees: vec!["alice".into(), "bob".into()],
    };

    let issue = generate_github(&server)
        .remove_issue_assignees(&http(), "token", &input)
        .await
        .unwrap();

    assert_eq!(issue.id, "151");
}

#[tokio::test]
async fn lists_a_page_of_the_people_issues_can_be_assigned_to() {
    let server = MockServer::start().await;
    let people = [generate_github_user(161), generate_github_user(162)];
    MockGithubNetwork::on(&server)
        .answer(
            "GET",
            "/repos/octo/hello/assignees",
            &[("page", "2")],
            200,
            json!(people),
        )
        .await;
    let input = ListAssignees {
        owner: "octo".into(),
        repo: "hello".into(),
        per_page: None,
        page: Some(2),
    };

    let listed = generate_github(&server)
        .list_assignees(&http(), "token", &input)
        .await
        .unwrap();

    assert_eq!(listed[1].id, "162");
    assert_eq!(listed[1].login, text(&people[1], "login"));
    assert_eq!(listed[1].url, text(&people[1], "html_url"));
    assert_eq!(listed[1].account_type, "User");
}

// Locks.

#[rstest]
#[case::with_a_reason(Some("too heated"), json!({ "lock_reason": "too heated" }))]
#[case::without_a_reason(None, json!({}))]
#[tokio::test]
async fn locks_an_issue_s_conversation(#[case] reason: Option<&str>, #[case] sent: Value) {
    let server = MockServer::start().await;
    MockGithubIssuesNetwork::on(&server)
        .answer_body("PUT", "/repos/octo/hello/issues/8/lock", sent, 204, None)
        .await;
    let input = LockIssue {
        owner: "octo".into(),
        repo: "hello".into(),
        issue_number: 8,
        lock_reason: reason.map(str::to_owned),
    };

    let lock = generate_github(&server)
        .lock_issue(&http(), "token", &input)
        .await
        .unwrap();

    assert!(lock.locked);
}

#[tokio::test]
async fn unlocks_an_issue_s_conversation() {
    let server = MockServer::start().await;
    MockGithubIssuesNetwork::on(&server)
        .answer_empty("DELETE", "/repos/octo/hello/issues/8/lock", 204)
        .await;
    let input = UnlockIssue {
        owner: "octo".into(),
        repo: "hello".into(),
        issue_number: 8,
    };

    let lock = generate_github(&server)
        .unlock_issue(&http(), "token", &input)
        .await
        .unwrap();

    assert!(!lock.locked);
}

// Comments.

#[tokio::test]
async fn lists_a_page_of_an_issue_s_comments() {
    let server = MockServer::start().await;
    let comments = [generate_github_issue_comment(171)];
    MockGithubNetwork::on(&server)
        .answer(
            "GET",
            "/repos/octo/hello/issues/2/comments",
            &[("per_page", "5"), ("page", "3")],
            200,
            json!(comments),
        )
        .await;
    let input = ListIssueComments {
        owner: "octo".into(),
        repo: "hello".into(),
        issue_number: 2,
        per_page: Some(5),
        page: Some(3),
    };

    let listed = generate_github(&server)
        .list_issue_comments(&http(), "token", &input)
        .await
        .unwrap();

    assert_eq!(listed[0].id, "171");
    assert_eq!(listed[0].url, text(&comments[0], "html_url"));
    assert_eq!(listed[0].body, text(&comments[0], "body"));
    assert_eq!(listed[0].user.as_ref().unwrap().id, "172");
}

#[tokio::test]
async fn comments_on_an_issue() {
    let server = MockServer::start().await;
    let body = word();
    MockGithubIssuesNetwork::on(&server)
        .answer_body(
            "POST",
            "/repos/octo/hello/issues/2/comments",
            json!({ "body": body }),
            201,
            Some(generate_github_issue_comment(181)),
        )
        .await;
    let input = CreateIssueComment {
        owner: "octo".into(),
        repo: "hello".into(),
        issue_number: 2,
        body,
    };

    let comment = generate_github(&server)
        .create_issue_comment(&http(), "token", &input)
        .await
        .unwrap();

    assert_eq!(comment.id, "181");
}

#[tokio::test]
async fn reads_a_comment_on_an_issue_by_id() {
    let server = MockServer::start().await;
    let answer = generate_github_issue_comment(191);
    MockGithubNetwork::on(&server)
        .answer(
            "GET",
            "/repos/octo/hello/issues/comments/191",
            &[],
            200,
            answer.clone(),
        )
        .await;
    let input = GetIssueComment {
        owner: "octo".into(),
        repo: "hello".into(),
        comment_id: 191,
    };

    let comment = generate_github(&server)
        .get_issue_comment(&http(), "token", &input)
        .await
        .unwrap();

    assert_eq!(comment.id, "191");
    assert_eq!(comment.created_at, text(&answer, "created_at"));
}

#[tokio::test]
async fn replaces_a_comment_s_text() {
    let server = MockServer::start().await;
    let body = word();
    MockGithubIssuesNetwork::on(&server)
        .answer_body(
            "PATCH",
            "/repos/octo/hello/issues/comments/201",
            json!({ "body": body }),
            200,
            Some(generate_github_issue_comment(201)),
        )
        .await;
    let input = UpdateIssueComment {
        owner: "octo".into(),
        repo: "hello".into(),
        comment_id: 201,
        body,
    };

    let comment = generate_github(&server)
        .update_issue_comment(&http(), "token", &input)
        .await
        .unwrap();

    assert_eq!(comment.id, "201");
}

#[tokio::test]
async fn deletes_a_comment_on_an_issue() {
    let server = MockServer::start().await;
    MockGithubIssuesNetwork::on(&server)
        .answer_empty("DELETE", "/repos/octo/hello/issues/comments/211", 204)
        .await;
    let input = DeleteIssueComment {
        owner: "octo".into(),
        repo: "hello".into(),
        comment_id: 211,
    };

    let done = generate_github(&server)
        .delete_issue_comment(&http(), "token", &input)
        .await
        .unwrap();

    assert!(done.ok);
}

// Events.

#[tokio::test]
async fn lists_an_issue_s_timeline_including_entries_with_no_id_of_their_own() {
    let server = MockServer::start().await;
    let commit = generate_github_timeline_commit();
    MockGithubNetwork::on(&server)
        .answer(
            "GET",
            "/repos/octo/hello/issues/2/timeline",
            &[("per_page", "2")],
            200,
            json!([generate_github_timeline_comment(221), commit]),
        )
        .await;
    let input = ListIssueTimelineEvents {
        owner: "octo".into(),
        repo: "hello".into(),
        issue_number: 2,
        per_page: Some(2),
        page: None,
    };

    let timeline = generate_github(&server)
        .list_issue_timeline_events(&http(), "token", &input)
        .await
        .unwrap();

    assert_eq!(timeline[0].id.as_deref(), Some("221"));
    assert_eq!(timeline[0].event, "commented");
    assert_eq!(timeline[0].actor.as_ref().unwrap().id, "222");
    assert_eq!(timeline[1].id, None);
    assert_eq!(timeline[1].event, "committed");
    assert_eq!(timeline[1].actor, None);
    assert_eq!(timeline[1].created_at, None);
}

#[tokio::test]
async fn lists_a_page_of_what_happened_to_an_issue() {
    let server = MockServer::start().await;
    let events = [generate_github_issue_event(231, "labeled")];
    MockGithubNetwork::on(&server)
        .answer(
            "GET",
            "/repos/octo/hello/issues/2/events",
            &[("page", "4")],
            200,
            json!(events),
        )
        .await;
    let input = ListIssueEvents {
        owner: "octo".into(),
        repo: "hello".into(),
        issue_number: 2,
        per_page: None,
        page: Some(4),
    };

    let listed = generate_github(&server)
        .list_issue_events(&http(), "token", &input)
        .await
        .unwrap();

    assert_eq!(listed[0].id, "231");
    assert_eq!(listed[0].event, "labeled");
    assert_eq!(listed[0].actor.as_ref().unwrap().id, "232");
    assert_eq!(
        listed[0].commit_id.as_deref(),
        events[0]["commit_id"].as_str()
    );
}

#[tokio::test]
async fn lists_a_page_of_what_happened_to_a_repository_s_issues() {
    let server = MockServer::start().await;
    MockGithubNetwork::on(&server)
        .answer(
            "GET",
            "/repos/octo/hello/issues/events",
            &[("per_page", "50")],
            200,
            json!([
                generate_github_issue_event(241, "closed"),
                generate_github_issue_event(242, "reopened"),
            ]),
        )
        .await;
    let input = ListRepositoryIssueEvents {
        owner: "octo".into(),
        repo: "hello".into(),
        per_page: Some(50),
        page: None,
    };

    let listed = generate_github(&server)
        .list_repository_issue_events(&http(), "token", &input)
        .await
        .unwrap();

    assert_eq!(listed.len(), 2);
    assert_eq!(listed[1].id, "242");
    assert_eq!(listed[1].event, "reopened");
}

// Milestones.

#[tokio::test]
async fn lists_a_page_of_milestones_filtered_and_sorted() {
    let server = MockServer::start().await;
    let milestones = [generate_github_milestone(251, 1)];
    MockGithubNetwork::on(&server)
        .answer(
            "GET",
            "/repos/octo/hello/milestones",
            &[
                ("state", "closed"),
                ("sort", "completeness"),
                ("direction", "desc"),
                ("per_page", "20"),
                ("page", "1"),
            ],
            200,
            json!(milestones),
        )
        .await;
    let input = ListMilestones {
        owner: "octo".into(),
        repo: "hello".into(),
        state: Some("closed".into()),
        sort: Some("completeness".into()),
        direction: Some("desc".into()),
        per_page: Some(20),
        page: Some(1),
    };

    let listed = generate_github(&server)
        .list_milestones(&http(), "token", &input)
        .await
        .unwrap();

    assert_eq!(listed[0].id, "251");
    assert_eq!(listed[0].url, text(&milestones[0], "html_url"));
    assert_eq!(
        listed[0].open_issues,
        milestones[0]["open_issues"].as_i64().unwrap()
    );
}

#[tokio::test]
async fn reads_a_milestone_by_number() {
    let server = MockServer::start().await;
    let answer = generate_github_milestone(261, 3);
    MockGithubNetwork::on(&server)
        .answer(
            "GET",
            "/repos/octo/hello/milestones/3",
            &[],
            200,
            answer.clone(),
        )
        .await;
    let input = GetMilestone {
        owner: "octo".into(),
        repo: "hello".into(),
        milestone_number: 3,
    };

    let milestone = generate_github(&server)
        .get_milestone(&http(), "token", &input)
        .await
        .unwrap();

    assert_eq!(milestone.id, "261");
    assert_eq!(milestone.number, 3);
    assert_eq!(milestone.title, text(&answer, "title"));
}

#[tokio::test]
async fn creates_a_milestone_with_only_the_fields_given() {
    let server = MockServer::start().await;
    let title = word();
    MockGithubIssuesNetwork::on(&server)
        .answer_body(
            "POST",
            "/repos/octo/hello/milestones",
            json!({ "title": title, "due_on": "2026-12-31T00:00:00Z" }),
            201,
            Some(generate_github_milestone(271, 4)),
        )
        .await;
    let input = CreateMilestone {
        owner: "octo".into(),
        repo: "hello".into(),
        title,
        state: None,
        description: None,
        due_on: Some("2026-12-31T00:00:00Z".into()),
    };

    let milestone = generate_github(&server)
        .create_milestone(&http(), "token", &input)
        .await
        .unwrap();

    assert_eq!(milestone.id, "271");
}

#[tokio::test]
async fn creates_a_milestone_with_its_state_trimmed_and_a_blank_due_date_left_out() {
    let server = MockServer::start().await;
    let title = word();
    MockGithubIssuesNetwork::on(&server)
        .answer_body(
            "POST",
            "/repos/octo/hello/milestones",
            json!({ "title": title, "state": "closed", "description": "  kept as given  " }),
            201,
            Some(generate_github_milestone(272, 4)),
        )
        .await;
    let input = CreateMilestone {
        owner: "octo".into(),
        repo: "hello".into(),
        title,
        state: Some(" closed ".into()),
        description: Some("  kept as given  ".into()),
        due_on: Some("  ".into()),
    };

    let milestone = generate_github(&server)
        .create_milestone(&http(), "token", &input)
        .await
        .unwrap();

    assert_eq!(milestone.id, "272");
}

#[tokio::test]
async fn updates_a_milestone_with_only_the_fields_given() {
    let server = MockServer::start().await;
    MockGithubIssuesNetwork::on(&server)
        .answer_body(
            "PATCH",
            "/repos/octo/hello/milestones/4",
            json!({ "state": "closed", "description": "Shipped" }),
            200,
            Some(generate_github_milestone(281, 4)),
        )
        .await;
    let input = UpdateMilestone {
        owner: "octo".into(),
        repo: "hello".into(),
        milestone_number: 4,
        title: None,
        state: Some("closed".into()),
        description: Some("Shipped".into()),
        due_on: None,
    };

    let milestone = generate_github(&server)
        .update_milestone(&http(), "token", &input)
        .await
        .unwrap();

    assert_eq!(milestone.id, "281");
}

#[tokio::test]
async fn deletes_a_milestone() {
    let server = MockServer::start().await;
    MockGithubIssuesNetwork::on(&server)
        .answer_empty("DELETE", "/repos/octo/hello/milestones/4", 204)
        .await;
    let input = DeleteMilestone {
        owner: "octo".into(),
        repo: "hello".into(),
        milestone_number: 4,
    };

    let done = generate_github(&server)
        .delete_milestone(&http(), "token", &input)
        .await
        .unwrap();

    assert!(done.ok);
}

// Reactions.

#[tokio::test]
async fn reacts_to_an_issue() {
    let server = MockServer::start().await;
    MockGithubIssuesNetwork::on(&server)
        .answer_body(
            "POST",
            "/repos/octo/hello/issues/2/reactions",
            json!({ "content": "+1" }),
            201,
            Some(generate_github_reaction(291, "+1")),
        )
        .await;
    let input = CreateIssueReaction {
        owner: "octo".into(),
        repo: "hello".into(),
        issue_number: 2,
        content: "+1".into(),
    };

    let reaction = generate_github(&server)
        .create_issue_reaction(&http(), "token", &input)
        .await
        .unwrap();

    assert_eq!(reaction.id, "291");
    assert_eq!(reaction.content, "+1");
    assert_eq!(reaction.user.unwrap().id, "292");
}

#[tokio::test]
async fn reacts_to_a_comment_on_an_issue() {
    let server = MockServer::start().await;
    MockGithubIssuesNetwork::on(&server)
        .answer_body(
            "POST",
            "/repos/octo/hello/issues/comments/301/reactions",
            json!({ "content": "rocket" }),
            201,
            Some(generate_github_reaction(302, "rocket")),
        )
        .await;
    let input = CreateIssueCommentReaction {
        owner: "octo".into(),
        repo: "hello".into(),
        comment_id: 301,
        content: "rocket".into(),
    };

    let reaction = generate_github(&server)
        .create_issue_comment_reaction(&http(), "token", &input)
        .await
        .unwrap();

    assert_eq!(reaction.id, "302");
    assert_eq!(reaction.content, "rocket");
}

// Search.

#[tokio::test]
async fn searches_issues_and_pull_requests_with_the_filters_as_qualifiers() {
    let server = MockServer::start().await;
    let items = [
        generate_github_search_issue(311, 1, false),
        generate_github_search_issue(312, 2, true),
    ];
    MockGithubNetwork::on(&server)
        .answer(
            "GET",
            "/search/issues",
            &[
                (
                    "q",
                    "crash repo:octo/hello state:open label:\"help wanted\"",
                ),
                ("sort", "comments"),
                ("order", "asc"),
                ("per_page", "2"),
                ("page", "1"),
            ],
            200,
            json!({ "total_count": 40, "incomplete_results": true, "items": items }),
        )
        .await;
    let input = SearchIssuesAndPullRequests {
        query: Some("crash".into()),
        owner: Some("octo".into()),
        repo: Some("hello".into()),
        state: Some("open".into()),
        label: Some("help wanted".into()),
        sort: Some("comments".into()),
        order: Some("asc".into()),
        per_page: Some(2),
        page: Some(1),
        ..Default::default()
    };

    let found = generate_github(&server)
        .search_issues_and_pull_requests(&http(), "token", &input)
        .await
        .unwrap();

    assert_eq!(found.total_count, 40);
    assert!(found.incomplete_results);
    assert_eq!(found.items.len(), 2);
    assert_eq!(found.items[0].id, "311");
    assert_eq!(found.items[0].url, text(&items[0], "html_url"));
    assert_eq!(
        found.items[0].repository_url,
        "https://api.github.com/repos/octo/hello"
    );
    assert!(!found.items[0].is_pull_request);
    assert!(found.items[1].is_pull_request);
}

#[tokio::test]
async fn reads_a_search_answer_without_items_as_nothing_found() {
    let server = MockServer::start().await;
    MockGithubNetwork::on(&server)
        .answer("GET", "/search/issues", &[("q", "")], 200, json!({}))
        .await;

    let found = generate_github(&server)
        .search_issues_and_pull_requests(&http(), "token", &SearchIssuesAndPullRequests::default())
        .await
        .unwrap();

    assert_eq!(found.total_count, 0);
    assert!(!found.incomplete_results);
    assert!(found.items.is_empty());
}
