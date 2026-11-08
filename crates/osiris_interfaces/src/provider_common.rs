//! What every provider is, whatever it does: its id, where its API is, how a request carries the
//! token, and how its errors read.

use crate::{ProviderError, status_error::status_error};
use reqwest::{
    StatusCode,
    header::{AUTHORIZATION, HeaderName},
};
use url::Url;

pub trait ProviderCommon: Send + Sync {
    /// Its key, a row of the `provider` table.
    fn id(&self) -> &'static str;

    /// Its API's base URL, from `[providers]` in the settings.
    fn base_url(&self) -> &Url;

    /// Whether a granted scope covers a `required` one: the same scope by default. A provider whose
    /// scopes nest (one covering narrower ones) overrides it.
    fn scope_covers(&self, granted: &str, required: &str) -> bool {
        granted == required
    }

    /// The header a request carries the token in. A bearer token by default.
    fn auth_header(&self, token: &str) -> (HeaderName, String) {
        (AUTHORIZATION, format!("Bearer {token}"))
    }

    /// An error answer as a [`ProviderError`], read by its status ([`status_error`]). A provider
    /// whose errors read otherwise overrides it.
    fn parse_error(&self, status: StatusCode, body: &str) -> ProviderError {
        status_error(self.id(), status, body)
    }
}
