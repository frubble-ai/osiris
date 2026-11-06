//! Reads the profile; there must be one.

use super::{Profile, path::path};
use anyhow::Context;

pub fn load() -> anyhow::Result<Profile> {
    let path = path()?;
    let text = std::fs::read_to_string(&path)
        .with_context(|| format!("no profile at {}: run `osiris login` first", path.display()))?;
    toml::from_str(&text).with_context(|| format!("reading {}", path.display()))
}
