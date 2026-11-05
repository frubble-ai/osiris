//! Tests of `refresh`.

use super::*;
use rstest::rstest;
use std::time::Duration;

const THIRTY_DAYS: Duration = Duration::from_hours(30 * 24);

#[test]
fn accepts_a_keepalive_under_the_schedulers_limit_or_a_scheduler_without_one() {
    let settings = RefreshSettings::default();
    assert!(settings.check_against(Some(THIRTY_DAYS)).is_ok());
    assert!(settings.check_against(None).is_ok());
}

#[rstest]
#[case::at_the_limit(THIRTY_DAYS.as_secs())]
#[case::over_the_limit(THIRTY_DAYS.as_secs() + 1)]
fn refuses_a_keepalive_the_scheduler_could_not_schedule(#[case] seconds: u64) {
    let settings = RefreshSettings {
        keepalive_seconds: seconds,
        ..RefreshSettings::default()
    };

    let error = settings.check_against(Some(THIRTY_DAYS)).unwrap_err();

    assert!(error.to_string().contains("refresh.keepalive_seconds"));
}
