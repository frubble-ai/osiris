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
mod tests;
