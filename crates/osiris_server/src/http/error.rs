//! Every error the API answers: `{ "error": { "code", "type", "message" } }` with its status. A
//! domain error's report is logged with its whole chain; an internal one is answered without it.

use axum::{
    Json,
    http::StatusCode,
    response::{IntoResponse, Response},
};
use error_stack::Report;
use osiris_api_models::errors::{ErrorBody, ErrorDetail};
use osiris_domain::DomainError;

#[derive(Debug)]
pub enum ApiError {
    Unauthorized,
    Invalid(String),
    UnknownVersion(String),
    TooLarge,
    IdempotencyMismatch,
    InFlight,
    Domain(Report<DomainError>),
}

impl From<Report<DomainError>> for ApiError {
    fn from(report: Report<DomainError>) -> Self {
        Self::Domain(report)
    }
}

impl ApiError {
    fn parts(&self) -> (StatusCode, &'static str, &'static str, String) {
        match self {
            Self::Unauthorized => (
                StatusCode::UNAUTHORIZED,
                "OS_00",
                "unauthorized",
                "a valid key is required".into(),
            ),
            Self::Invalid(message) => (
                StatusCode::BAD_REQUEST,
                "OS_02",
                "invalid_request",
                message.clone(),
            ),
            Self::UnknownVersion(version) => (
                StatusCode::BAD_REQUEST,
                "OS_07",
                "unknown_version",
                format!("Osiris-Version {version:?} is not one osiris serves"),
            ),
            Self::TooLarge => (
                StatusCode::PAYLOAD_TOO_LARGE,
                "OS_10",
                "too_large",
                "the body is larger than the server reads".into(),
            ),
            Self::IdempotencyMismatch => (
                StatusCode::UNPROCESSABLE_ENTITY,
                "OS_08",
                "idempotency_mismatch",
                "this Idempotency-Key was used for another request".into(),
            ),
            Self::InFlight => (
                StatusCode::CONFLICT,
                "OS_09",
                "in_flight",
                "the first request with this Idempotency-Key is still running".into(),
            ),
            Self::Domain(report) => {
                let error = report.current_context();
                let message = error.to_string();
                match error {
                    DomainError::NotFound(_) => {
                        (StatusCode::NOT_FOUND, "OS_01", "not_found", message)
                    }
                    DomainError::Invalid(_) => {
                        (StatusCode::BAD_REQUEST, "OS_02", "invalid_request", message)
                    }
                    DomainError::Conflict(_) => {
                        (StatusCode::CONFLICT, "OS_03", "conflict", message)
                    }
                    DomainError::ProviderRefused(_) => (
                        StatusCode::FAILED_DEPENDENCY,
                        "OS_04",
                        "provider_refused",
                        // The provider's answer stays in the logged report.
                        "the provider refused the request".into(),
                    ),
                    DomainError::ProviderUnavailable(_) => (
                        StatusCode::BAD_GATEWAY,
                        "OS_05",
                        "provider_unavailable",
                        // The provider's answer stays in the logged report.
                        "the provider could not be reached; try again later".into(),
                    ),
                    DomainError::NotActive(_) => (
                        StatusCode::CONFLICT,
                        "OS_06",
                        "connection_not_active",
                        message,
                    ),
                    DomainError::MissingScopes(_) => {
                        (StatusCode::FORBIDDEN, "OS_11", "missing_scopes", message)
                    }
                    DomainError::Internal => (
                        StatusCode::INTERNAL_SERVER_ERROR,
                        "OS_99",
                        "internal",
                        message,
                    ),
                }
            }
        }
    }
}

impl IntoResponse for ApiError {
    fn into_response(self) -> Response {
        let (status, code, error_type, message) = self.parts();
        if let Self::Domain(report) = &self {
            if status.is_server_error() {
                tracing::error!(error = ?report, code, "request failed");
            } else {
                tracing::info!(error = %report.current_context(), code, "request refused");
            }
        }
        let body = ErrorBody {
            error: ErrorDetail {
                code: code.into(),
                error_type: error_type.into(),
                message,
            },
        };
        (status, Json(body)).into_response()
    }
}

pub type ApiResult<T> = Result<T, ApiError>;
