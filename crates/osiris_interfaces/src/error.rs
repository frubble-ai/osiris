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
    /// The input can't be sent as given (a field the connection's kind of token can't use), or it
    /// names something the action doesn't read (a directory asked for as a file). The caller
    /// changes it; the reason is answered.
    #[error("{provider} can't take the input: {message}")]
    Invalid {
        provider: &'static str,
        message: String,
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

    /// Whether the provider answered 404: what it asked about isn't there (or isn't shown to the
    /// token), which some actions read as an answer rather than a failure.
    pub fn is_not_found(&self) -> bool {
        self.status() == Some(404)
    }

    /// The HTTP status a refusal came with, if it came as one.
    pub fn status(&self) -> Option<u16> {
        match self {
            Self::Refused { status, .. } => *status,
            _ => None,
        }
    }

    pub fn invalid(provider: &'static str, message: impl Into<String>) -> Self {
        Self::Invalid {
            provider,
            message: message.into(),
        }
    }

    pub fn unavailable(provider: &'static str, error: impl std::fmt::Display) -> Self {
        Self::Unavailable {
            provider,
            message: error.to_string(),
        }
    }
}
