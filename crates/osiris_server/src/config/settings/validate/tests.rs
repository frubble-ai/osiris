//! `validate` refuses each setting that can't work, naming it.

use super::super::Settings;
use rstest::rstest;

fn broken(change: impl FnOnce(&mut Settings)) -> String {
    let mut settings = Settings::default();
    change(&mut settings);
    settings.validate().unwrap_err().to_string()
}

#[test]
fn accepts_the_defaults() {
    Settings::default().validate().unwrap();
}

#[rstest]
#[case::a_zero_value("server.max_body_bytes", |s: &mut Settings| s.server.max_body_bytes = 0)]
#[case::a_zero_timeout("http.timeout_seconds", |s: &mut Settings| s.http.timeout_seconds = 0)]
#[case::a_short_consent_state("consent.state_length", |s: &mut Settings| s.consent.state_length = 31)]
#[case::more_concurrent_tasks_than_connections(
    "tasks.max_concurrent",
    |s: &mut Settings| s.tasks.max_concurrent = usize::try_from(s.database.max_connections).unwrap() + 1
)]
#[case::a_margin_past_the_keepalive(
    "refresh.margin_seconds",
    |s: &mut Settings| s.refresh.margin_seconds = s.refresh.keepalive_seconds
)]
#[case::a_default_page_longer_than_the_longest(
    "pagination.default_limit",
    |s: &mut Settings| s.pagination.default_limit = s.pagination.max_limit + 1
)]
#[case::an_unknown_api_version("api.pinned_version", |s: &mut Settings| s.api.pinned_version = "1999-01-01.nope".into())]
#[case::a_provider_url_that_is_not_a_url(
    "providers.github",
    |s: &mut Settings| s.providers.github.api_base_url = "not a url".into()
)]
fn refuses_a_setting_that_cannot_work_naming_it(
    #[case] name: &str,
    #[case] change: fn(&mut Settings),
) {
    let error = broken(change);

    assert!(error.contains(name), "{error}");
}
