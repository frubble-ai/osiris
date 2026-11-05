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
mod tests {
    use super::*;
    use config::{File, FileFormat};

    #[test]
    fn the_defaults_are_valid() {
        Settings::default().validate().unwrap();
    }

    #[test]
    fn every_documented_key_is_a_setting_and_matches_its_default() {
        let example: Settings = config::Config::builder()
            .add_source(File::from_str(
                include_str!("../../../../config.example.toml"),
                FileFormat::Toml,
            ))
            .build()
            .and_then(config::Config::try_deserialize)
            .unwrap();
        example.validate().unwrap();
        // config.example.toml documents the defaults, so the two must agree.
        assert_eq!(
            serde_json::to_value(&example).unwrap(),
            serde_json::to_value(Settings::default()).unwrap()
        );
    }
}
