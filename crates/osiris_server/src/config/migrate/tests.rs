//! Tests of `migrate`.

use super::{MigrateConfig, reset_requested};
use crate::config::Settings;
use rstest::rstest;

const PEM: &str = "-----BEGIN CERTIFICATE-----\nMIIB\n-----END CERTIFICATE-----";

fn from_vars(reset: Option<&str>, reset_database: Option<&str>) -> anyhow::Result<MigrateConfig> {
    MigrateConfig::from_vars(
        &|key| match key {
            "DB_HOST" => Some("localhost".to_owned()),
            "DB_PORT" => Some("5433".to_owned()),
            "DB_NAME" | "DB_USER" => Some("osiris".to_owned()),
            "DB_PASSWORD" => Some("secret".to_owned()),
            "DB_CA_PEM" => Some(PEM.to_owned()),
            "OSIRIS_MIGRATE_RESET" => reset.map(str::to_owned),
            "OSIRIS_MIGRATE_RESET_DATABASE" => reset_database.map(str::to_owned),
            _ => None,
        },
        &Settings::default(),
    )
}

#[rstest]
#[case::exact(Some("true"), true)]
#[case::unset(None, false)]
#[case::empty(Some(""), false)]
#[case::one(Some("1"), false)]
#[case::upper(Some("TRUE"), false)]
#[case::false_(Some("false"), false)]
fn reset_is_only_the_exact_string_true(#[case] value: Option<&str>, #[case] reset: bool) {
    assert_eq!(reset_requested(value), reset);
}

#[rstest]
#[case::matches(true, Some("osiris"), "osiris", true)]
#[case::not_resetting(false, None, "osiris", true)]
#[case::missing(true, None, "osiris", false)]
#[case::blank(true, Some("  "), "osiris", false)]
#[case::other_database(true, Some("other"), "osiris", false)]
fn reset_names_the_connected_database(
    #[case] reset: bool,
    #[case] target: Option<&str>,
    #[case] connected: &str,
    #[case] allowed: bool,
) {
    assert_eq!(
        super::require_reset_target(reset, target, connected).is_ok(),
        allowed
    );
}

#[rstest]
#[case::matching(Some("true"), Some("osiris"), true)]
#[case::missing_name(Some("true"), None, false)]
#[case::not_resetting(None, None, true)]
fn from_env_reads_the_reset_and_the_database_name(
    #[case] reset: Option<&str>,
    #[case] reset_database: Option<&str>,
    #[case] allowed: bool,
) {
    let result = from_vars(reset, reset_database);
    assert_eq!(result.is_ok(), allowed);
    if let Ok(config) = result {
        assert_eq!(config.reset, reset == Some("true"));
        assert_eq!(config.reset_database.as_deref(), reset_database);
        assert_eq!(config.database.name, "osiris");
    }
}
