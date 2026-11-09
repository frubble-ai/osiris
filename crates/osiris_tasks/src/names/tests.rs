//! Tests of `names`.

use super::*;
use rstest::rstest;

#[test]
fn names_follow_what_the_task_is_for() {
    assert_eq!(refresh("ca_1", 3), refresh("ca_1", 3));
    assert_ne!(refresh("ca_1", 3), refresh("ca_1", 4));
    assert_ne!(refresh("ca_1", 3), refresh("ca_2", 3));
    assert_ne!(refresh("ca_1", 3), refresh_now("ca_1", 3));
    assert_ne!(trigger_poll("tr_1", 1), trigger_poll("tr_1", 2));
    assert_ne!(trigger_poll("tr_1", 1), trigger_poll("tr_2", 1));
    assert_eq!(consent("ca_1"), "consent-expire-ca_1");
    assert_eq!(event("ev_9"), "event-publish-ev_9");
}

#[rstest]
#[case::refresh(refresh("ca_Ab9", 12))]
#[case::refresh_now(refresh_now("ca_Ab9", 1))]
#[case::consent(consent("ca_x"))]
#[case::event(event("ev_y"))]
#[case::trigger_poll(trigger_poll("tr_Ab9", 1))]
fn names_use_only_what_schedulers_accept(#[case] name: String) {
    assert!(
        name.chars()
            .all(|character| character.is_ascii_alphanumeric()
                || character == '-'
                || character == '_')
    );
    assert!(name.len() <= 500);
}
