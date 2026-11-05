//! Which process the image runs, from `OSIRIS_PROCESS`: `serve`, `worker` or `migrate`. It is
//! required and has no default, so the platform never sets a command or arguments, and a missing
//! or misspelled value stops the image at once instead of starting the wrong process.

use anyhow::bail;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Process {
    /// The public API and the OAuth callback.
    Serve,
    /// The internal service the task scheduler sends scheduled tasks to.
    Worker,
    /// Applies the pending migrations, then exits.
    Migrate,
}

impl Process {
    pub fn from_env() -> anyhow::Result<Self> {
        Self::parse(super::env_vars::optional("OSIRIS_PROCESS").as_deref())
    }

    pub fn parse(value: Option<&str>) -> anyhow::Result<Self> {
        match value.map(str::trim) {
            Some("serve") => Ok(Self::Serve),
            Some("worker") => Ok(Self::Worker),
            Some("migrate") => Ok(Self::Migrate),
            Some(other) => {
                bail!("OSIRIS_PROCESS must be serve, worker or migrate, not {other:?}")
            }
            None => bail!("OSIRIS_PROCESS is required: serve, worker or migrate"),
        }
    }
}

#[cfg(test)]
mod tests;
