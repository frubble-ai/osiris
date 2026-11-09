//! Tests of `triggers`.

use super::*;
use rstest::rstest;
use std::time::Duration;

const THIRTY_DAYS: Duration = Duration::from_hours(30 * 24);

#[test]
fn accepts_a_check_interval_under_the_schedulers_limit_or_a_scheduler_without_one() {
    let settings = TriggersSettings::default();
    assert!(settings.check_against(Some(THIRTY_DAYS)).is_ok());
    assert!(settings.check_against(None).is_ok());
}

#[rstest]
#[case::at_the_limit(THIRTY_DAYS.as_secs())]
#[case::over_the_limit(THIRTY_DAYS.as_secs() + 1)]
fn refuses_a_check_interval_the_scheduler_could_not_schedule(#[case] seconds: u64) {
    let settings = TriggersSettings {
        check_seconds: seconds,
        ..TriggersSettings::default()
    };

    let error = settings.check_against(Some(THIRTY_DAYS)).unwrap_err();

    assert!(error.to_string().contains("triggers.check_seconds"));
}
