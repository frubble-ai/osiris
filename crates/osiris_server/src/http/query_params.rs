//! A route's query. One that can't be read as the route's type (a limit that isn't a number, or is
//! out of range) is refused as `OS_02` in the error shape every answer uses, never as axum's
//! plain-text rejection. The message is osiris's own, so an axum or serde upgrade that rewords its
//! errors changes no answer.

use super::error::ApiError;
use axum::{
    extract::{FromRequestParts, Query},
    http::request::Parts,
};
use serde::de::DeserializeOwned;

pub struct QueryParams<T>(pub T);

impl<S, T> FromRequestParts<S> for QueryParams<T>
where
    S: Send + Sync,
    T: DeserializeOwned,
{
    type Rejection = ApiError;

    async fn from_request_parts(parts: &mut Parts, state: &S) -> Result<Self, Self::Rejection> {
        Query::<T>::from_request_parts(parts, state)
            .await
            .map(|Query(query)| Self(query))
            .map_err(|_| {
                ApiError::Invalid(
                    "the query does not match the request: a parameter has the wrong type".into(),
                )
            })
    }
}
