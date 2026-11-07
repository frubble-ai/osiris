//! A route's JSON body. A body that can't be read as the route's type (not JSON, a missing field,
//! a wrong type, no JSON content type) is refused as `OS_02` in the error shape every answer uses,
//! never as axum's plain-text rejection.

use super::error::ApiError;
use axum::{
    Json,
    extract::{FromRequest, Request},
};
use serde::de::DeserializeOwned;

pub struct JsonBody<T>(pub T);

impl<S, T> FromRequest<S> for JsonBody<T>
where
    S: Send + Sync,
    T: DeserializeOwned,
{
    type Rejection = ApiError;

    async fn from_request(request: Request, state: &S) -> Result<Self, Self::Rejection> {
        Json::<T>::from_request(request, state)
            .await
            .map(|Json(body)| Self(body))
            .map_err(|rejection| ApiError::Invalid(rejection.body_text()))
    }
}
