//! Tests of `refresh`.

use super::*;
use std::time::Duration;

const THIRTY_DAYS: Duration = Duration::from_hours(30 * 24);

#[test]
fn accepts_a_keepalive_under_the_schedulers_limit_or_a_scheduler_without_one() {
    let settings = RefreshSettings::default();
    assert!(settings.check_against(Some(THIRTY_DAYS)).is_ok());
    assert!(settings.check_against(None).is_ok());
}

#[test]
fn refuses_a_keepalive_the_scheduler_could_not_schedule() {
    for seconds in [THIRTY_DAYS.as_secs(), THIRTY_DAYS.as_secs() + 1] {
        let settings = RefreshSettings {
            keepalive_seconds: seconds,
            ..RefreshSettings::default()
        };
        let error = settings.check_against(Some(THIRTY_DAYS)).unwrap_err();
        assert!(
            error.to_string().contains("refresh.keepalive_seconds"),
            "{seconds}"
        );
    }
}
