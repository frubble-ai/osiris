//! The changes since a release run in order: oldest first to bring a request up, newest first to
//! take an answer, an event or the spec back down; none for the current release or one not listed.

use super::{changes_since, event_down, query_up, request_up, response_down, spec_down};
use crate::{Release, Route, mocks::MOCK_RELEASES};
use rstest::rstest;
use serde_json::{Value, json};

const ROUTE: Route<'static> = Route {
    method: "GET",
    path: "/v1/things",
};

const NOT_LISTED: Release = Release {
    name: "2027-01-01.horus",
    number: 9,
    stability: crate::Stability::Preview,
    changes: &[],
};

fn trail(value: &Value) -> Vec<&str> {
    value["trail"]
        .as_array()
        .map(|marks| marks.iter().filter_map(Value::as_str).collect())
        .unwrap_or_default()
}

#[rstest]
#[case::the_first(0, &["a", "b", "c"])]
#[case::the_middle(1, &["b", "c"])]
#[case::the_current(2, &[])]
fn brings_a_request_up_oldest_first(#[case] target: usize, #[case] marks: &[&str]) {
    let mut body = json!({});
    request_up(MOCK_RELEASES, &MOCK_RELEASES[target], &ROUTE, &mut body);
    assert_eq!(trail(&body), marks);

    let mut query = Vec::new();
    query_up(MOCK_RELEASES, &MOCK_RELEASES[target], &ROUTE, &mut query);
    let found: Vec<&str> = query.iter().map(|(_, mark)| mark.as_str()).collect();
    assert_eq!(found, marks);
}

#[rstest]
#[case::the_first(0, &["c", "b", "a"])]
#[case::the_middle(1, &["c", "b"])]
#[case::the_current(2, &[])]
fn takes_an_answer_an_event_and_the_spec_down_newest_first(
    #[case] target: usize,
    #[case] marks: &[&str],
) {
    let release = &MOCK_RELEASES[target];
    let mut answer = json!({});
    response_down(MOCK_RELEASES, release, &ROUTE, 200, &mut answer);
    assert_eq!(trail(&answer), marks);

    let mut event = json!({});
    event_down(MOCK_RELEASES, release, &mut event);
    assert_eq!(trail(&event), marks);

    let mut spec = json!({ "info": {} });
    spec_down(MOCK_RELEASES, release, &mut spec);
    assert_eq!(trail(&spec), marks);
    assert_eq!(spec["info"]["version"], release.name);
    assert_eq!(spec["info"]["x-osiris-release-number"], release.number);
}

#[test]
fn makes_no_change_for_a_release_not_listed() {
    assert_eq!(changes_since(MOCK_RELEASES, &NOT_LISTED).count(), 0);
}
