//! Tests of `names`.

use super::*;

#[test]
fn names_follow_what_the_task_is_for() {
    assert_eq!(refresh("ca_1", 3), refresh("ca_1", 3));
    assert_ne!(refresh("ca_1", 3), refresh("ca_1", 4));
    assert_ne!(refresh("ca_1", 3), refresh("ca_2", 3));
    assert_ne!(refresh("ca_1", 3), refresh_now("ca_1", 3));
    assert_eq!(consent("ca_1"), "consent-expire-ca_1");
    assert_eq!(event("ev_9"), "event-publish-ev_9");
}

#[test]
fn names_use_only_what_schedulers_accept() {
    for name in [
        refresh("ca_Ab9", 12),
        refresh_now("ca_Ab9", 1),
        consent("ca_x"),
        event("ev_y"),
    ] {
        assert!(
            name.chars()
                .all(|c| c.is_ascii_alphanumeric() || c == '-' || c == '_')
        );
        assert!(name.len() <= 500);
    }
}
