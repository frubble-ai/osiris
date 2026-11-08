//! A token an app's installation acts with, minted from the app's own credentials.

/// An installation's token and until when it is good.
#[derive(Debug, Clone)]
pub struct InstallationToken {
    pub token: String,
    /// When the provider stops accepting it, as the provider wrote it (RFC 3339).
    pub expires_at: String,
}
