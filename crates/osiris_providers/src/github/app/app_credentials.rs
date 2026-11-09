//! A GitHub App's credentials, read from an auth config's settings: `app_id` in `settings`, and
//! `private_key` (the PEM file GitHub downloads, PKCS#1 or PKCS#8) in `secret_settings`.

use serde_json::Value;

/// The app's id and private key.
pub struct AppCredentials {
    /// The app's numeric id, or its client id (`Iv1.…`), as its settings page shows.
    pub app_id: String,
    /// The private key, in PEM.
    pub private_key: String,
}

/// The PEM headers GitHub's private keys come with.
const PEM_HEADERS: &[&str] = &[
    "-----BEGIN RSA PRIVATE KEY-----",
    "-----BEGIN PRIVATE KEY-----",
];

impl AppCredentials {
    /// The credentials `settings` and `secret_settings` hold, or why they don't. A key pasted with
    /// its line breaks escaped (`\n`) reads as the PEM it is.
    pub fn read(settings: &Value, secret_settings: Option<&Value>) -> Result<Self, String> {
        let app_id = settings
            .get("app_id")
            .and_then(Value::as_str)
            .map(str::trim)
            .unwrap_or_default();
        if app_id.is_empty()
            || !app_id
                .chars()
                .all(|character| character.is_ascii_alphanumeric() || character == '.')
        {
            return Err(
                "settings.app_id must be the GitHub App's id or client id, from its settings page"
                    .into(),
            );
        }
        let private_key = secret_settings
            .and_then(|secret| secret.get("private_key"))
            .and_then(Value::as_str)
            .map(|key| key.replace("\\n", "\n").trim().to_owned())
            .unwrap_or_default();
        if !PEM_HEADERS
            .iter()
            .any(|header| private_key.starts_with(header))
        {
            return Err("secret_settings.private_key must be the GitHub App's private key, the PEM file its settings page downloads".into());
        }
        Ok(Self {
            app_id: app_id.to_owned(),
            private_key,
        })
    }
}
