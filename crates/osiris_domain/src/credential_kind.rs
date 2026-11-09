//! What a connection acts as: rows of `credential_kind`.

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum CredentialKind {
    /// The user who consented, with the tokens their consent gave.
    #[serde(rename = "oauth")]
    OAuth,
    /// An app's installation, with tokens minted on demand from the app's own credentials and
    /// never stored.
    AppInstallation,
}

impl CredentialKind {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::OAuth => "oauth",
            Self::AppInstallation => "app_installation",
        }
    }

    pub fn parse(value: &str) -> Option<Self> {
        match value {
            "oauth" => Some(Self::OAuth),
            "app_installation" => Some(Self::AppInstallation),
            _ => None,
        }
    }
}
