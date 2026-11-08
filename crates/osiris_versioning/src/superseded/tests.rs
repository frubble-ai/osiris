//! A release is superseded on the next release's date, and its support ends the window after; the
//! newest release is neither.

use crate::{mocks::MOCK_RELEASES, superseded};
use time::{Date, Duration, Month};

#[test]
fn supersedes_a_release_on_the_next_one_s_date_until_the_window_ends() {
    let (since, sunset) = superseded(MOCK_RELEASES, &MOCK_RELEASES[0], Duration::days(30)).unwrap();

    assert_eq!(
        since.date(),
        Date::from_calendar_date(2026, Month::February, 1).unwrap()
    );
    assert_eq!(
        sunset.date(),
        Date::from_calendar_date(2026, Month::March, 3).unwrap()
    );
    assert!(superseded(MOCK_RELEASES, &MOCK_RELEASES[2], Duration::days(30)).is_none());
}
