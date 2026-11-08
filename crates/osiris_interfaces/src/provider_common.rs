//! What every provider is, whatever it does: its id, where its API is, how a request carries the
//! token, and how its errors read.

use crate::ProviderError;
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
    /// scopes nest (GitHub's `repo` covers `repo:status`) overrides it.
    fn scope_covers(&self, granted: &str, required: &str) -> bool {
        granted == required
    }

    /// The header a request carries the token in. A bearer token by default.
    fn auth_header(&self, token: &str) -> (HeaderName, String) {
        (AUTHORIZATION, format!("Bearer {token}"))
    }

    /// An error answer as a [`ProviderError`]: 401 is a rejected token, another 4xx a refusal,
    /// the rest a failure. A provider whose errors read otherwise overrides it.
    fn parse_error(&self, status: StatusCode, body: &str) -> ProviderError {
        let message = format!("{status}: {body}");
        match status {
            StatusCode::UNAUTHORIZED => ProviderError::unauthorized(self.id(), message),
            status if status.is_client_error() => {
                ProviderError::refused_with_status(self.id(), status.as_u16(), message)
            }
            _ => ProviderError::unavailable(self.id(), message),
        }
    }
}
