//! A route's JSON body. A body that can't be read as the route's type (not JSON, a missing field,
//! a wrong type, no JSON content type) is refused as `OS_02` in the error shape every answer uses,
//! never as axum's plain-text rejection; a body too large is `OS_10` (413). The message is osiris's own, so an axum or serde upgrade
//! that rewords its errors changes no answer.

use super::error::ApiError;
use axum::{
    Json,
    extract::{FromRequest, Request, rejection::JsonRejection},
    http::StatusCode,
};
use serde::de::DeserializeOwned;

pub struct JsonBody<T>(pub T);

impl<S, T> FromRequest<S> for JsonBody<T>
where
    S: Send + Sync,
    T: DeserializeOwned,
{
    type Rejection = ApiError;

    /// Reads the body as `T`, or refuses it as `OS_02` naming why.
    async fn from_request(request: Request, state: &S) -> Result<Self, Self::Rejection> {
        Json::<T>::from_request(request, state)
            .await
            .map(|Json(body)| Self(body))
            .map_err(|rejection| {
                // A body past `[server] max_body_bytes` keeps its 413.
                if rejection.status() == StatusCode::PAYLOAD_TOO_LARGE {
                    ApiError::TooLarge
                } else {
                    ApiError::Invalid(message(&rejection).into())
                }
            })
    }
}

/// The `OS_02` message for why a body couldn't be read, in osiris's words.
fn message(rejection: &JsonRejection) -> &'static str {
    match rejection {
        JsonRejection::MissingJsonContentType(_) => "the body must be sent as application/json",
        JsonRejection::JsonSyntaxError(_) => "the body is not valid JSON",
        JsonRejection::JsonDataError(_) => {
            "the body does not match the request: a field is missing or has the wrong type"
        }
        _ => "the body could not be read",
    }
}
