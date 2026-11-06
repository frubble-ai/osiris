//! Secrets in memory: tokens, client secrets, keys. [`Secret`] never prints its value (Debug shows
//! `[REDACTED]`), and reading it takes an explicit `expose_secret()`.

pub use secrecy::{ExposeSecret, SecretString as Secret};

/// A secret from a plain string.
pub fn secret(value: impl Into<String>) -> Secret {
    Secret::from(value.into())
}

#[cfg(test)]
mod tests;
