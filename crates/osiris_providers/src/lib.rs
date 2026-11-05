//! The providers osiris implements, one module each: the provider itself (`ProviderCommon`,
//! `Provider`), our types, its wire format and the conversions (`transformers`), its revoke, and
//! one module per action. The rest of osiris finds a provider by id through [`Providers`]; only
//! its per-provider action folders (core, routes) and the lists that register providers name one.

pub mod github;
pub mod linear;
pub mod list_limits;
pub mod provider_urls;
pub mod slack;

pub use list_limits::ListLimits;
pub use osiris_interfaces::{OAuthClient, Provider, ProviderCommon, ProviderError, Tokens};
pub use provider_urls::ProviderUrls;

/// Every provider osiris implements, at the URLs the settings give.
pub struct Providers {
    pub github: github::Github,
    pub linear: linear::Linear,
    pub slack: slack::Slack,
}

impl Providers {
    pub fn new(
        github: &ProviderUrls,
        linear: &ProviderUrls,
        slack: &ProviderUrls,
        limits: ListLimits,
    ) -> Result<Self, url::ParseError> {
        Ok(Self {
            github: github::Github::new(github, limits)?,
            linear: linear::Linear::new(linear, limits)?,
            slack: slack::Slack::new(slack, limits)?,
        })
    }

    /// The provider whose id is `id`.
    pub fn get(&self, id: &str) -> Option<&dyn Provider> {
        let all: [&dyn Provider; 3] = [&self.github, &self.linear, &self.slack];
        all.into_iter().find(|provider| provider.id() == id)
    }
}
