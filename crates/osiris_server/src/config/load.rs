//! Loads the settings: the defaults in code, overridden by `OSIRIS__<SECTION>__<KEY>` environment
//! variables. No file is read, and nothing depends on which environment osiris runs in. The
//! settings are validated before anything starts.

use super::Settings;
use config::Environment;

pub fn load() -> anyhow::Result<Settings> {
    let settings: Settings = config::Config::builder()
        .add_source(config::Config::try_from(&Settings::default())?)
        .add_source(
            Environment::with_prefix("OSIRIS")
                .prefix_separator("__")
                .separator("__")
                .try_parsing(true),
        )
        .build()
        .and_then(config::Config::try_deserialize)?;
    settings.validate()?;
    Ok(settings)
}

#[cfg(test)]
mod tests;
