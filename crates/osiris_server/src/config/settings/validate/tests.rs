//! `validate` refuses each setting that can't work, naming it.

use super::super::{Settings, providers::ProviderUrlSettings};
use osiris_providers::REGISTRATIONS;
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

#[test]
fn accepts_more_concurrent_tasks_than_connections() {
    let mut settings = Settings::default();
    settings.tasks.max_concurrent = usize::try_from(settings.database.max_connections).unwrap() + 1;

    settings.validate().unwrap();
}

#[rstest]
#[case::a_zero_value("server.max_body_bytes", |settings: &mut Settings| settings.server.max_body_bytes = 0)]
#[case::a_zero_timeout("http.timeout_seconds", |settings: &mut Settings| settings.http.timeout_seconds = 0)]
#[case::a_short_consent_state("consent.state_length", |settings: &mut Settings| settings.consent.state_length = 31)]
#[case::more_tasks_than_a_semaphore_holds(
    "tasks.max_concurrent",
    |settings: &mut Settings| settings.tasks.max_concurrent = tokio::sync::Semaphore::MAX_PERMITS + 1
)]
#[case::a_margin_past_the_keepalive(
    "refresh.margin_seconds",
    |settings: &mut Settings| settings.refresh.margin_seconds = settings.refresh.keepalive_seconds
)]
#[case::a_default_page_longer_than_the_longest(
    "pagination.default_limit",
    |settings: &mut Settings| settings.pagination.default_limit = settings.pagination.max_limit + 1
)]
#[case::no_support_window("api.support_days", |settings: &mut Settings| settings.api.support_days = 0)]
#[case::a_support_window_too_long_to_compute(
    "api.support_days",
    |settings: &mut Settings| settings.api.support_days = 999_999_999_999_999
)]
#[case::an_unknown_api_version("api.pinned_version", |settings: &mut Settings| settings.api.pinned_version = "1999-01-01.nope".into())]
fn refuses_a_setting_that_cannot_work_naming_it(
    #[case] name: &str,
    #[case] change: fn(&mut Settings),
) {
    let error = broken(change);

    assert!(error.contains(name), "{error}");
}

#[test]
fn saturates_a_support_window_too_long_to_compute_instead_of_overflowing() {
    let mut settings = Settings::default();
    settings.api.support_days = u64::MAX;

    assert_eq!(settings.api.support(), time::Duration::MAX);
}

#[test]
fn refuses_a_provider_url_that_is_not_a_url_naming_its_provider() {
    let id = REGISTRATIONS[0].id;
    let error = broken(|settings| {
        settings.providers.at.insert(
            id.to_owned(),
            ProviderUrlSettings {
                oauth_base_url: None,
                api_base_url: Some("not a url".into()),
            },
        );
    });

    assert!(error.contains(&format!("providers.{id}")), "{error}");
}

#[test]
fn refuses_a_section_that_is_no_provider() {
    let error = broken(|settings| {
        settings
            .providers
            .at
            .insert("listng".into(), ProviderUrlSettings::default());
    });

    assert!(error.contains("providers.listng"), "{error}");
}
