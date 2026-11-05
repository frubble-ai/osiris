//! The CLI's profile: where osiris is and the keys to call it with, in
//! `$XDG_CONFIG_HOME/osiris/config.toml` (or `~/.config/osiris/config.toml`), written by `login`
//! and readable by its owner only.

use anyhow::Context;
use serde::{Deserialize, Serialize};
use std::{io::Write, path::PathBuf};

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct Profile {
    /// Osiris's origin, e.g. `http://localhost:8090`.
    pub url: String,
    pub api_key: Option<String>,
    pub admin_key: Option<String>,
    /// For `listen`: the Pub/Sub project and events topic, and the emulator locally.
    pub pubsub_project: Option<String>,
    pub events_topic: Option<String>,
    pub pubsub_emulator_host: Option<String>,
}

pub fn path() -> anyhow::Result<PathBuf> {
    let base = match std::env::var_os("XDG_CONFIG_HOME") {
        Some(dir) => PathBuf::from(dir),
        None => PathBuf::from(std::env::var_os("HOME").context("HOME is not set")?).join(".config"),
    };
    Ok(base.join("osiris").join("config.toml"))
}

pub fn load() -> anyhow::Result<Profile> {
    let path = path()?;
    let text = std::fs::read_to_string(&path)
        .with_context(|| format!("no profile at {}: run `osiris login` first", path.display()))?;
    toml::from_str(&text).with_context(|| format!("reading {}", path.display()))
}

/// The saved profile, or `None` when there is none yet. A profile that exists but can't be read
/// is an error.
pub fn load_if_present() -> anyhow::Result<Option<Profile>> {
    let path = path()?;
    match std::fs::read_to_string(&path) {
        Ok(text) => toml::from_str(&text)
            .map(Some)
            .with_context(|| format!("reading {}", path.display())),
        Err(error) if error.kind() == std::io::ErrorKind::NotFound => Ok(None),
        Err(error) => Err(error).with_context(|| format!("reading {}", path.display())),
    }
}

pub fn save(profile: &Profile) -> anyhow::Result<PathBuf> {
    let path = path()?;
    if let Some(dir) = path.parent() {
        let mut builder = std::fs::DirBuilder::new();
        builder.recursive(true);
        #[cfg(unix)]
        std::os::unix::fs::DirBuilderExt::mode(&mut builder, 0o700);
        builder.create(dir)?;
    }
    let mut options = std::fs::OpenOptions::new();
    options.write(true).create(true).truncate(true);
    // Created owner-only, so the keys are never readable by anyone else, not even for a moment.
    #[cfg(unix)]
    std::os::unix::fs::OpenOptionsExt::mode(&mut options, 0o600);
    let mut file = options.open(&path)?;
    // A file made before with looser permissions is tightened before the keys go in.
    #[cfg(unix)]
    {
        use std::os::unix::fs::PermissionsExt;
        file.set_permissions(std::fs::Permissions::from_mode(0o600))?;
    }
    file.write_all(toml::to_string_pretty(profile)?.as_bytes())?;
    Ok(path)
}
