//! The providers osiris implements, one module each: the provider itself (`ProviderCommon`,
//! `Provider`), our types, its wire format and the conversions (`transformers`), its revoke, and
//! one module per action. The rest of osiris finds a provider by id through [`Providers`]; only
//! its per-provider action models and routes and the lists that register providers name one.

pub mod github;
pub mod linear;
pub mod list_limits;
#[cfg(any(test, feature = "testing"))]
pub mod mocks;
pub mod provider_urls;
pub mod providers;
pub mod slack;

pub use list_limits::ListLimits;
pub use osiris_interfaces::{
    InstallationToken, OAuthClient, Provider, ProviderCommon, ProviderError, Tokens,
};
pub use provider_urls::ProviderUrls;
pub use providers::Providers;
