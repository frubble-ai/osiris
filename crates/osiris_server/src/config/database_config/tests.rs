//! Tests of `database_config`.

use super::*;
use std::collections::HashMap;

const CA: &str = "-----BEGIN CERTIFICATE-----\nMIIB\n-----END CERTIFICATE-----\n";

fn vars() -> HashMap<&'static str, String> {
    HashMap::from([
        ("DB_HOST", "db.internal".to_owned()),
        ("DB_PORT", "5432".to_owned()),
        ("DB_NAME", "osiris".to_owned()),
        ("DB_USER", "osiris".to_owned()),
        ("DB_PASSWORD", "secret".to_owned()),
        ("DB_CA_PEM", CA.to_owned()),
    ])
}

fn read(vars: &HashMap<&'static str, String>) -> anyhow::Result<DatabaseConfig> {
    DatabaseConfig::from_vars(&|name| vars.get(name).cloned(), &Settings::default())
}

#[test]
fn builds_a_verify_ca_connection_from_the_db_variables() {
    let config = read(&vars()).unwrap();
    let options = config.connect_options();
    assert_eq!(options.get_host(), "db.internal");
    assert_eq!(options.get_port(), 5432);
    assert_eq!(options.get_database(), Some("osiris"));
    assert_eq!(options.get_username(), "osiris");
    assert!(matches!(options.get_ssl_mode(), PgSslMode::VerifyCa));
    assert_eq!(
        config.max_connections,
        Settings::default().database.max_connections
    );
}

#[test]
fn refuses_to_start_without_any_db_variable() {
    for name in [
        "DB_HOST",
        "DB_PORT",
        "DB_NAME",
        "DB_USER",
        "DB_PASSWORD",
        "DB_CA_PEM",
    ] {
        for missing in [None, Some("  ")] {
            let mut vars = vars();
            match missing {
                None => vars.remove(name),
                Some(blank) => vars.insert(name, blank.to_owned()),
            };
            let error = read(&vars).err().unwrap().to_string();
            assert!(error.contains(name), "{name}: {error}");
        }
    }
}

#[test]
fn refuses_a_port_that_is_not_a_number_and_a_ca_that_is_not_pem() {
    let mut bad_port = vars();
    bad_port.insert("DB_PORT", "five".to_owned());
    assert!(
        read(&bad_port)
            .err()
            .unwrap()
            .to_string()
            .contains("DB_PORT")
    );
    let mut bad_ca = vars();
    bad_ca.insert("DB_CA_PEM", "not a certificate".to_owned());
    assert!(
        read(&bad_ca)
            .err()
            .unwrap()
            .to_string()
            .contains("DB_CA_PEM")
    );
}
