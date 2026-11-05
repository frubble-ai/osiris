//! Writes the profile, readable by its owner only: the file is created owner-only (and an older
//! one tightened) before the keys go in, in a directory only its owner can enter.

use super::{Profile, path::path};
use std::{
    io::Write,
    path::{Path, PathBuf},
};

pub fn save(profile: &Profile) -> anyhow::Result<PathBuf> {
    let path = path()?;
    save_to(&path, profile)?;
    Ok(path)
}

/// Writes `profile` to `path`, owner-only.
pub fn save_to(path: &Path, profile: &Profile) -> anyhow::Result<()> {
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
    let mut file = options.open(path)?;
    // A file made before with looser permissions is tightened before the keys go in.
    #[cfg(unix)]
    {
        use std::os::unix::fs::PermissionsExt;
        file.set_permissions(std::fs::Permissions::from_mode(0o600))?;
    }
    file.write_all(toml::to_string_pretty(profile)?.as_bytes())?;
    Ok(())
}
