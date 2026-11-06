//! What one run of the migrate process does, read once at boot. The database comes from the
//! `DB_*` variables. `OSIRIS_MIGRATE_RESET=true` exactly drops the tables and enums this database
//! user owns before migrating. Any other value, including unset, only migrates.
//!
//! The reset workflow sets the variable for one Cloud Run execution. The job's own environment
//! never has it, so a deploy's migration does not reset. The process does not read which
//! environment it is running in: prd is refused by the workflow and by the platform's trust,
//! which names `db-reset.yml` only for dev and stg.

use super::DatabaseConfig;
use super::Settings;

#[derive(Clone)]
pub struct MigrateConfig {
    pub database: DatabaseConfig,
    /// Drop this database user's tables and enums before applying the migrations.
    pub reset: bool,
}

impl MigrateConfig {
    pub fn from_env(settings: &Settings) -> anyhow::Result<Self> {
        Ok(Self {
            database: DatabaseConfig::from_env(settings)?,
            reset: reset_requested(super::env_vars::optional("OSIRIS_MIGRATE_RESET").as_deref()),
        })
    }
}

/// Only the exact string `true` resets. `TRUE`, `1` and an empty value do not.
fn reset_requested(value: Option<&str>) -> bool {
    value == Some("true")
}

#[cfg(test)]
mod tests;
