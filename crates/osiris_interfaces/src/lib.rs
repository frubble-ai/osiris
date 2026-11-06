//! What every provider implements and shares: [`ProviderCommon`] (who it is, where it is, how it
//! is called and how it fails), [`Provider`] (its `OAuth2` and how it ends a grant), the `OAuth2` flow
//! driven by a provider's definition, and [`call`], the one way a provider's API is called.

pub mod call;
pub mod error;
pub mod oauth;
pub mod oauth_client;
pub mod provider;
pub mod provider_common;
pub mod tokens;

pub use call::call;
pub use error::ProviderError;
pub use oauth_client::OAuthClient;
pub use provider::Provider;
pub use provider_common::ProviderCommon;
pub use tokens::Tokens;
