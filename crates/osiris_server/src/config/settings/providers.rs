//! `[providers.<id>]`: where each provider is. Tests and a local fake point these elsewhere.

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(default, deny_unknown_fields)]
pub struct ProvidersSettings {
    pub github: ProviderUrlSettings,
    pub linear: ProviderUrlSettings,
    pub slack: ProviderUrlSettings,
    pub listing: ListingSettings,
    pub installation_tokens: InstallationTokenSettings,
}

/// `[providers.installation_tokens]`: how long an app installation's token is reused.
#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(default, deny_unknown_fields)]
pub struct InstallationTokenSettings {
    /// A token is minted again this long before the provider stops accepting it.
    pub margin_seconds: u64,
}

impl InstallationTokenSettings {
    pub fn margin(&self) -> time::Duration {
        time::Duration::seconds(i64::try_from(self.margin_seconds).unwrap_or(i64::MAX))
    }
}

impl Default for InstallationTokenSettings {
    fn default() -> Self {
        // GitHub's installation tokens live an hour; five minutes before, a new one is minted.
        Self {
            margin_seconds: 300,
        }
    }
}

/// `[providers.listing]`: how far a list that fans out over a provider's resources goes.
#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(default, deny_unknown_fields)]
pub struct ListingSettings {
    /// Resources read at once (e.g. GitHub installations).
    pub concurrency: usize,
    /// Items one list answers at most.
    pub max_items: usize,
}

impl ListingSettings {
    pub fn limits(&self) -> osiris_providers::ListLimits {
        osiris_providers::ListLimits {
            concurrency: self.concurrency,
            max_items: self.max_items,
        }
    }
}

impl Default for ListingSettings {
    fn default() -> Self {
        Self {
            concurrency: 4,
            max_items: 5000,
        }
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct ProviderUrlSettings {
    /// Where the consent page is.
    pub oauth_base_url: String,
    /// Where the API is.
    pub api_base_url: String,
}

impl ProviderUrlSettings {
    fn at(oauth_base_url: &str, api_base_url: &str) -> Self {
        Self {
            oauth_base_url: oauth_base_url.to_owned(),
            api_base_url: api_base_url.to_owned(),
        }
    }

    pub fn urls(&self) -> Result<osiris_providers::ProviderUrls, url::ParseError> {
        Ok(osiris_providers::ProviderUrls {
            oauth_base: self.oauth_base_url.parse()?,
            api_base: self.api_base_url.parse()?,
        })
    }
}

impl Default for ProvidersSettings {
    fn default() -> Self {
        Self {
            github: ProviderUrlSettings::at("https://github.com", "https://api.github.com"),
            linear: ProviderUrlSettings::at("https://linear.app", "https://api.linear.app"),
            slack: ProviderUrlSettings::at("https://slack.com", "https://slack.com/api/"),
            listing: ListingSettings::default(),
            installation_tokens: InstallationTokenSettings::default(),
        }
    }
}
