//! What one run of the migrate process does, read once at boot. The database comes from the
//! `DB_*` variables. `OSIRIS_MIGRATE_RESET=true` exactly drops the tables and enums this database
//! user owns in `public` before migrating. Any other value, including unset, only migrates.
//!
//! The reset workflow sets the variable for one Cloud Run execution. The job's own environment
//! never has it, so a deploy's migration does not reset. The process does not read which
//! environment it is running in: prd is refused by the workflow and by the platform's trust,
//! which names `db-reset.yml` only for dev and stg.

use super::DatabaseConfig;
use super::Settings;
use anyhow::bail;

/// One migrate run: where the database is, and whether this run drops first.
#[derive(Clone)]
pub struct MigrateConfig {
    pub database: DatabaseConfig,
    /// Drop this database user's tables and enums in `public` before applying the migrations.
    pub reset: bool,
    /// The database a reset must be connected to. Required when `reset` is set, and it
    /// must be that connection's database. A reset without it is refused.
    pub reset_database: Option<String>,
}

impl MigrateConfig {
    /// Reads `DB_*`, `OSIRIS_MIGRATE_RESET` and `OSIRIS_MIGRATE_RESET_DATABASE`.
    pub fn from_env(settings: &Settings) -> anyhow::Result<Self> {
        Self::from_vars(&super::env_vars::optional, settings)
    }

    /// Reads the variables through `lookup`, so tests can supply them without the process
    /// environment.
    pub fn from_vars(
        lookup: &dyn Fn(&str) -> Option<String>,
        settings: &Settings,
    ) -> anyhow::Result<Self> {
        let database = DatabaseConfig::from_vars(lookup, settings)?;
        let reset = reset_requested(lookup("OSIRIS_MIGRATE_RESET").as_deref());
        let reset_database = lookup("OSIRIS_MIGRATE_RESET_DATABASE");
        require_reset_target(reset, reset_database.as_deref(), &database.name)?;
        Ok(Self {
            database,
            reset,
            reset_database,
        })
    }
}

/// Only the exact string `true` resets. `TRUE`, `1` and an empty value do not.
fn reset_requested(value: Option<&str>) -> bool {
    value == Some("true")
}

/// A reset must name the database it is about to drop, and that name must be the one
/// this connection is on. Anything else is refused before a table is dropped.
pub(crate) fn require_reset_target(
    reset: bool,
    target: Option<&str>,
    connected: &str,
) -> anyhow::Result<()> {
    if !reset {
        return Ok(());
    }
    match target.map(str::trim).filter(|name| !name.is_empty()) {
        Some(name) if name == connected => Ok(()),
        Some(name) => bail!(
            "OSIRIS_MIGRATE_RESET_DATABASE is {name:?}, the connected database is {connected:?}"
        ),
        None => {
            bail!("OSIRIS_MIGRATE_RESET_DATABASE is required when OSIRIS_MIGRATE_RESET=true")
        }
    }
}

#[cfg(test)]
mod tests;
