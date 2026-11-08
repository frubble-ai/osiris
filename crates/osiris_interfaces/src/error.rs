//! What a provider call can end in.

#[derive(Debug, thiserror::Error)]
pub enum ProviderError {
    /// The provider no longer accepts the token: the grant is gone there (revoked, uninstalled).
    #[error("{provider} rejected the token: {message}")]
    Unauthorized {
        provider: &'static str,
        message: String,
    },
    /// The provider refused: a bad code, a missing permission, a request it won't serve.
    #[error("{provider} refused: {message}")]
    Refused {
        provider: &'static str,
        message: String,
        /// The HTTP status it answered, when the refusal came as one.
        status: Option<u16>,
    },
    /// The provider could not be reached, or answered something we could not read.
    #[error("{provider} failed: {message}")]
    Unavailable {
        provider: &'static str,
        message: String,
    },
}

impl ProviderError {
    pub fn unauthorized(provider: &'static str, message: impl Into<String>) -> Self {
        Self::Unauthorized {
            provider,
            message: message.into(),
        }
    }

    pub fn refused(provider: &'static str, message: impl Into<String>) -> Self {
        Self::Refused {
            provider,
            message: message.into(),
            status: None,
        }
    }

    /// A refusal the provider answered with the HTTP status `status`.
    pub fn refused_with_status(
        provider: &'static str,
        status: u16,
        message: impl Into<String>,
    ) -> Self {
        Self::Refused {
            provider,
            message: message.into(),
            status: Some(status),
        }
    }

    /// The HTTP status a refusal came with, if it came as one.
    pub fn status(&self) -> Option<u16> {
        match self {
            Self::Refused { status, .. } => *status,
            _ => None,
        }
    }

    pub fn unavailable(provider: &'static str, error: impl std::fmt::Display) -> Self {
        Self::Unavailable {
            provider,
            message: error.to_string(),
        }
    }
}
