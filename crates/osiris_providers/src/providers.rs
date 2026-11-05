//! The providers osiris implements, found by id.

use crate::{ListLimits, Provider, ProviderUrls, github, linear, slack};

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
