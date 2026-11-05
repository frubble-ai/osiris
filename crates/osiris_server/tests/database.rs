//! The database connection, against the local Postgres that only accepts TLS: built from the
//! same `DB_*` variables as when deployed, with verify-ca against `DB_CA_PEM`. Run through `just
//! test` (or with the `DB_*` variables exported), as CI does.

#![allow(
    clippy::unwrap_used,
    clippy::expect_used,
    reason = "tests: a failure here is the test failing"
)]

use osiris_server::{
    config::{DatabaseConfig, Settings},
    db,
};

/// A CA certificate that did not sign the local Postgres's certificate.
const UNRELATED_CA: &str = include_str!("support/unrelated-ca.pem");

fn from_env(overrides: &[(&str, &str)]) -> anyhow::Result<DatabaseConfig> {
    let overrides: Vec<(String, String)> = overrides
        .iter()
        .map(|(name, value)| ((*name).to_owned(), (*value).to_owned()))
        .collect();
    DatabaseConfig::from_vars(
        &|name| {
            overrides
                .iter()
                .find(|(overridden, _)| overridden == name)
                .map(|(_, value)| value.clone())
                .or_else(|| std::env::var(name).ok())
        },
        &Settings::default(),
    )
}

#[tokio::test]
async fn connects_over_tls_verified_against_db_ca_pem() {
    let config = from_env(&[]).expect("the DB_* variables are set: run through `just test`");
    let pool = db::connect(&config).await.unwrap();
    let encrypted: bool =
        sqlx::query_scalar("SELECT ssl FROM pg_stat_ssl WHERE pid = pg_backend_pid()")
            .fetch_one(&pool)
            .await
            .unwrap();
    assert!(encrypted, "the connection uses TLS");
}

#[tokio::test]
async fn refuses_to_start_when_the_server_s_certificate_is_not_signed_by_db_ca_pem() {
    let config = from_env(&[("DB_CA_PEM", UNRELATED_CA)]).unwrap();
    let error = db::connect(&config).await.unwrap_err();
    assert!(format!("{error:#}").contains("TLS"), "{error:#}");
}

#[tokio::test]
async fn refuses_to_start_when_the_database_cannot_be_reached() {
    // A port nothing listens on: startup fails instead of waiting for the first request.
    let mut config = from_env(&[("DB_PORT", "1")]).unwrap();
    config.connect_timeout = std::time::Duration::from_secs(2);
    assert!(db::connect(&config).await.is_err());
}
