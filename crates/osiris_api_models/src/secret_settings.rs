//! An app's secret settings as a client sends them: a JSON object (a GitHub App's private key)
//! held as a secret from the moment it is read, so it is never logged or answered.

use osiris_common::secret::{Secret, secret};
use serde::{Deserialize, Deserializer, de::Error};
use serde_json::Value;

/// A JSON object, kept as a [`Secret`] holding its JSON text.
#[derive(Debug)]
pub struct SecretSettings(pub Secret);

impl<'de> Deserialize<'de> for SecretSettings {
    /// Reads a JSON object; anything else is refused.
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        match Value::deserialize(deserializer)? {
            object @ Value::Object(_) => Ok(Self(secret(object.to_string()))),
            _ => Err(D::Error::custom("secret_settings must be a JSON object")),
        }
    }
}
