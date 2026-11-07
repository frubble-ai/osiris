//! A release's date and god come from its name; a name that isn't `<date>.<god>` has neither.

use crate::{Release, Stability, mocks::MOCK_RELEASES, superseded};
use rstest::rstest;
use time::{Date, Duration, Month};

fn named(name: &'static str) -> Release {
    Release {
        name,
        number: 1,
        stability: Stability::Preview,
        changes: &[],
    }
}

#[rstest]
#[case::a_release("2026-11-04.anubis", Some((2026, Month::November, 4)), "anubis")]
#[case::no_date("anubis", None, "")]
#[case::a_month_that_does_not_exist("2026-13-01.ra", None, "ra")]
#[case::a_day_that_does_not_exist("2026-02-30.ra", None, "ra")]
#[case::too_short("2026-1", None, "")]
fn reads_a_release_s_date_and_god_from_its_name(
    #[case] name: &'static str,
    #[case] date: Option<(i32, Month, u8)>,
    #[case] god: &str,
) {
    let release = named(name);
    let expected =
        date.map(|(year, month, day)| Date::from_calendar_date(year, month, day).unwrap());
    assert_eq!(release.released_on(), expected);
    assert_eq!(release.god(), god);
}

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
