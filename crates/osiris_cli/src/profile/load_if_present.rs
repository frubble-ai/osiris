//! Reads the profile if there is one.

use super::{Profile, path::path};
use anyhow::Context;
use std::path::Path;

/// The saved profile, or `None` when there is none yet. A profile that exists but can't be read
/// is an error.
pub fn load_if_present() -> anyhow::Result<Option<Profile>> {
    read_if_present(&path()?)
}

pub fn read_if_present(path: &Path) -> anyhow::Result<Option<Profile>> {
    match std::fs::read_to_string(path) {
        Ok(text) => toml::from_str(&text)
            .map(Some)
            .with_context(|| format!("reading {}", path.display())),
        Err(error) if error.kind() == std::io::ErrorKind::NotFound => Ok(None),
        Err(error) => Err(error).with_context(|| format!("reading {}", path.display())),
    }
}
