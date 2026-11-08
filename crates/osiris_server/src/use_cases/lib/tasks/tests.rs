//! A trigger's next check is due its interval from now, however long the interval.

use super::next_trigger_check::next_trigger_check;
use time::{Duration, OffsetDateTime};

#[test]
fn names_a_check_after_its_number_and_dues_it_an_interval_from_now() {
    let before = OffsetDateTime::now_utc();

    let task = next_trigger_check("tr_1", "ca_1", 3, Duration::minutes(5));

    assert_eq!(task.name, "trigger-poll-tr_1-3");
    assert!(task.run_at >= before + Duration::minutes(5));
}

#[test]
fn dues_a_check_too_far_to_compute_at_the_latest_time_instead_of_overflowing() {
    let task = next_trigger_check("tr_1", "ca_1", 1, Duration::MAX);

    assert!(task.run_at > OffsetDateTime::now_utc());
}
