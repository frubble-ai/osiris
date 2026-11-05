//! Where the database is, from the platform's standard variables: `DB_HOST`, `DB_PORT`,
//! `DB_NAME`, `DB_USER`, `DB_PASSWORD` and `DB_CA_PEM` (the server's CA certificate, in PEM). The
//! connection always uses TLS with `verify-ca` against that CA, in every environment; there is no
//! plaintext fallback and no way to turn it off. A missing or malformed value stops the process
//! at startup, naming the variable.

use super::Settings;
use anyhow::{Context, bail};
use osiris_common::secret::{ExposeSecret, Secret, secret};
use sqlx::postgres::{PgConnectOptions, PgSslMode};

#[derive(Clone)]
pub struct DatabaseConfig {
    pub host: String,
    pub port: u16,
    pub name: String,
    pub user: String,
    pub password: Secret,
    /// The CA certificate the server's certificate must be signed by.
    pub ca_pem: String,
    pub max_connections: u32,
    pub connect_timeout: std::time::Duration,
}

impl DatabaseConfig {
    pub fn from_env(settings: &Settings) -> anyhow::Result<Self> {
        Self::from_vars(&super::env_vars::optional, settings)
    }

    /// Reads the variables through `lookup`, so tests can supply them without the process
    /// environment.
    pub fn from_vars(
        lookup: &dyn Fn(&str) -> Option<String>,
        settings: &Settings,
    ) -> anyhow::Result<Self> {
        let required = |name: &str| {
            lookup(name)
                .filter(|value| !value.trim().is_empty())
                .with_context(|| format!("{name} is required"))
        };
        let port = required("DB_PORT")?;
        let ca_pem = required("DB_CA_PEM")?;
        if !ca_pem.contains("-----BEGIN CERTIFICATE-----") {
            bail!("DB_CA_PEM must be a PEM certificate (-----BEGIN CERTIFICATE-----)");
        }
        Ok(Self {
            host: required("DB_HOST")?,
            port: port
                .trim()
                .parse()
                .with_context(|| format!("DB_PORT must be a port number, not {port:?}"))?,
            name: required("DB_NAME")?,
            user: required("DB_USER")?,
            password: secret(required("DB_PASSWORD")?),
            ca_pem,
            max_connections: settings.database.max_connections,
            connect_timeout: settings.database.connect_timeout(),
        })
    }

    /// The connection options: TLS with `verify-ca` against `DB_CA_PEM`, always.
    pub fn connect_options(&self) -> PgConnectOptions {
        PgConnectOptions::new()
            .host(&self.host)
            .port(self.port)
            .database(&self.name)
            .username(&self.user)
            .password(self.password.expose_secret())
            .ssl_mode(PgSslMode::VerifyCa)
            .ssl_root_cert_from_pem(self.ca_pem.clone().into_bytes())
    }
}

#[cfg(test)]
mod tests {
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
}
