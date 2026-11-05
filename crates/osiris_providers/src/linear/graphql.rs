//! One query to Linear's GraphQL API. Linear answers its errors in `errors`, beside a 200; an
//! authentication error there means the token is no longer accepted.

use super::{Linear, transformers::GraphqlAnswer};
use crate::{ProviderCommon, ProviderError};
use osiris_interfaces::call::{Request, call};
use reqwest::Method;
use serde_json::{Value, json};

pub async fn graphql<T: serde::de::DeserializeOwned>(
    linear: &Linear,
    http: &reqwest::Client,
    token: &str,
    query: &str,
    variables: Value,
) -> Result<T, ProviderError> {
    let body = json!({ "query": query, "variables": variables });
    let answer: GraphqlAnswer<T> = call(
        linear,
        http,
        token,
        Request {
            method: Method::POST,
            path: "/graphql",
            query: &[],
            headers: &[],
            body: Some(&body),
        },
    )
    .await?;
    if let Some(errors) = answer.errors.filter(|errors| !errors.is_empty()) {
        let unauthenticated = errors.iter().any(|error| {
            error
                .extensions
                .as_ref()
                .and_then(|extensions| extensions.code.as_deref())
                == Some("AUTHENTICATION_ERROR")
        });
        let message = errors
            .into_iter()
            .map(|error| error.message)
            .collect::<Vec<_>>()
            .join("; ");
        return Err(if unauthenticated {
            ProviderError::unauthorized(linear.id(), message)
        } else {
            ProviderError::refused(linear.id(), message)
        });
    }
    answer
        .data
        .ok_or_else(|| ProviderError::unavailable(linear.id(), "a GraphQL answer without data"))
}
