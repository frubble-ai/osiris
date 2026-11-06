//! Where the profile lives: `$XDG_CONFIG_HOME/osiris/config.toml`, or
//! `~/.config/osiris/config.toml`.

use anyhow::Context;
use std::path::PathBuf;

pub fn path() -> anyhow::Result<PathBuf> {
    let base = match std::env::var_os("XDG_CONFIG_HOME") {
        Some(dir) => PathBuf::from(dir),
        None => PathBuf::from(std::env::var_os("HOME").context("HOME is not set")?).join(".config"),
    };
    Ok(base.join("osiris").join("config.toml"))
}
