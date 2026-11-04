//! Configuration, read from the environment once at startup: a missing or malformed variable
//! stops the process with the variable's name.

use anyhow::{Context, bail};

/// Where the database is. The migration needs only this, so it never asks for the service's keys.
#[derive(Clone)]
pub struct DatabaseConfig {
    pub url: String,
}

impl DatabaseConfig {
    pub fn from_env() -> anyhow::Result<Self> {
        Ok(Self {
            url: required("DATABASE_URL")?,
        })
    }
}

/// The service's whole configuration.
#[derive(Clone)]
pub struct Config {
    pub database: DatabaseConfig,
    /// Where the service listens: `HOST`, and `PORT` as Cloud Run sets it.
    pub host: String,
    pub port: u16,
}

impl Config {
    pub fn from_env() -> anyhow::Result<Self> {
        Ok(Self {
            database: DatabaseConfig::from_env()?,
            host: optional("HOST").unwrap_or_else(|| "0.0.0.0".to_owned()),
            port: match optional("PORT") {
                Some(port) => port.parse().context("PORT must be a port number")?,
                None => 8090,
            },
        })
    }
}

fn optional(name: &str) -> Option<String> {
    std::env::var(name).ok().filter(|value| !value.is_empty())
}

fn required(name: &str) -> anyhow::Result<String> {
    match optional(name) {
        Some(value) => Ok(value),
        None => bail!("{name} is required"),
    }
}
