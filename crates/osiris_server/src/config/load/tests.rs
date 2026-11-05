//! Tests of `load`.

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
            include_str!("../../../../../config.example.toml"),
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
