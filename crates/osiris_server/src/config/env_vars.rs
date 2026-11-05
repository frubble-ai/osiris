//! Reading one environment variable. A secret comes back as a [`Secret`], so it never prints.

use anyhow::bail;
use osiris_common::secret::{Secret, secret as wrap};

pub fn optional(name: &str) -> Option<String> {
    std::env::var(name).ok().filter(|value| !value.is_empty())
}

pub fn required(name: &str) -> anyhow::Result<String> {
    match optional(name) {
        Some(value) => Ok(value),
        None => bail!("{name} is required"),
    }
}

/// A key or secret: required, and long enough not to be guessed.
pub fn secret(name: &str) -> anyhow::Result<Secret> {
    let value = required(name)?;
    if value.len() < 32 {
        bail!("{name} must be at least 32 characters");
    }
    Ok(wrap(value))
}

/// A secret that is not always needed, such as a local push token.
pub fn optional_secret(name: &str) -> Option<Secret> {
    optional(name).map(wrap)
}
