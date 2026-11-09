//! The search text a search for issues and pull requests sends: the caller's own text, then a
//! qualifier for each filter, as `OpenConnector` builds it.

#![allow(
    clippy::unwrap_used,
    clippy::indexing_slicing,
    reason = "tests: a failure here is the test failing"
)]

use super::search_query;
use crate::github::issues::SearchIssuesAndPullRequests;
use rstest::rstest;

#[rstest]
#[case::nothing_given(SearchIssuesAndPullRequests::default(), "")]
#[case::the_search_text_alone(
    SearchIssuesAndPullRequests { query: Some("crash on start".into()), ..Default::default() },
    "crash on start"
)]
#[case::q_when_query_is_left_out(
    SearchIssuesAndPullRequests { q: Some("is:open".into()), ..Default::default() },
    "is:open"
)]
#[case::query_over_q(
    SearchIssuesAndPullRequests { query: Some("first".into()), q: Some("second".into()), ..Default::default() },
    "first"
)]
#[case::q_when_query_is_only_spaces(
    SearchIssuesAndPullRequests { query: Some("  ".into()), q: Some("second".into()), ..Default::default() },
    "second"
)]
#[case::owner_and_repo(
    SearchIssuesAndPullRequests { owner: Some("octo".into()), repo: Some("hello".into()), ..Default::default() },
    "repo:octo/hello"
)]
#[case::a_full_repo_name_alone(
    SearchIssuesAndPullRequests { repo: Some("octo/hello".into()), ..Default::default() },
    "repo:octo/hello"
)]
#[case::a_bare_repo_name_alone_adds_nothing(
    SearchIssuesAndPullRequests { repo: Some("hello".into()), ..Default::default() },
    ""
)]
#[case::an_owner_alone(
    SearchIssuesAndPullRequests { owner: Some("octo".into()), ..Default::default() },
    "user:octo"
)]
#[case::a_state(
    SearchIssuesAndPullRequests { state: Some("closed".into()), ..Default::default() },
    "state:closed"
)]
#[case::every_state_adds_nothing(
    SearchIssuesAndPullRequests { state: Some("all".into()), ..Default::default() },
    ""
)]
#[case::only_issues(
    SearchIssuesAndPullRequests { kind: Some("issue".into()), ..Default::default() },
    "is:issue"
)]
#[case::only_pull_requests(
    SearchIssuesAndPullRequests { kind: Some("pr".into()), ..Default::default() },
    "is:pr"
)]
#[case::a_label_with_a_space_in_quotes(
    SearchIssuesAndPullRequests { label: Some("good first issue".into()), ..Default::default() },
    "label:\"good first issue\""
)]
#[case::merged(
    SearchIssuesAndPullRequests { is_merged: Some(true), ..Default::default() },
    "is:merged"
)]
#[case::unmerged(
    SearchIssuesAndPullRequests { is_merged: Some(false), ..Default::default() },
    "is:unmerged"
)]
#[case::every_filter_in_order(
    SearchIssuesAndPullRequests {
        query: Some("crash".into()),
        owner: Some(" octo ".into()),
        repo: Some("hello".into()),
        state: Some("open".into()),
        kind: Some("pr".into()),
        label: Some("bug".into()),
        author: Some("alice".into()),
        assignee: Some("bob".into()),
        mentions: Some("carol".into()),
        language: Some("rust".into()),
        base_branch: Some("main".into()),
        head_branch: Some("feature/login".into()),
        is_merged: Some(true),
        ..Default::default()
    },
    "crash repo:octo/hello state:open is:pr label:bug author:alice assignee:bob mentions:carol \
     language:rust base:main head:feature/login is:merged"
)]
fn builds_the_search_text_from_the_filters(
    #[case] input: SearchIssuesAndPullRequests,
    #[case] expected: &str,
) {
    assert_eq!(search_query(&input), expected);
}
