//! Where a connection stands: rows of `connection_status`.

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ConnectionStatus {
    /// The provider's consent is open.
    Pending,
    /// Osiris holds the tokens, and their refresh is scheduled.
    Active,
    /// Not usable: the connection's provider issues refresh tokens, but osiris holds no scheduled
    /// refresh for it. At connect (the provider sent no refresh token, or the first refresh could
    /// not be scheduled) no tokens are kept and the user must connect again. After a refresh whose
    /// next task could not be scheduled, the new tokens are kept, and the refresh is scheduled
    /// again on the task's retry or the next action, which makes the connection active again.
    Disabled,
    /// The provider refused a refresh for good, or the grant is gone there: the user must
    /// reconnect.
    Expired,
    /// The consent was refused, or its code could not be exchanged.
    Failed,
}

impl ConnectionStatus {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Pending => "pending",
            Self::Active => "active",
            Self::Disabled => "disabled",
            Self::Expired => "expired",
            Self::Failed => "failed",
        }
    }

    pub fn parse(value: &str) -> Option<Self> {
        match value {
            "pending" => Some(Self::Pending),
            "active" => Some(Self::Active),
            "disabled" => Some(Self::Disabled),
            "expired" => Some(Self::Expired),
            "failed" => Some(Self::Failed),
            _ => None,
        }
    }
}
