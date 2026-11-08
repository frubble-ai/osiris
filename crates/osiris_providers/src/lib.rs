//! The providers osiris implements, one module each: the provider itself (`ProviderCommon`,
//! `Provider`), our types, its wire format and the conversions (`transformers`), its revoke, and
//! one module per action. Each registers itself from its folder (`registration.rs`, listed by
//! `build.rs`), and the rest of osiris finds it by id or type through [`Providers`].

pub mod github;
pub mod linear;
pub mod list_limits;
#[cfg(any(test, feature = "testing"))]
pub mod mocks;
pub mod provider_urls;
pub mod providers;
pub mod registration;
pub mod slack;

pub use list_limits::ListLimits;
pub use osiris_interfaces::{
    ActingAs, InstallationToken, OAuthClient, Provider, ProviderCommon, ProviderError, Tokens,
    TriggerCheck, TriggerReceipt,
};
pub use provider_urls::ProviderUrls;
pub use providers::Providers;
pub use registration::{REGISTRATIONS, Registered, Registration};
