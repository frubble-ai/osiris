//! `[providers]`: how far provider calls go, and `[providers.<id>]`, where a provider is when
//! not where it registers by default (tests and a local fake point it elsewhere).

use osiris_providers::{ProviderUrls, Registration};
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(default)]
pub struct ProvidersSettings {
    pub listing: ListingSettings,
    pub installation_tokens: InstallationTokenSettings,
    /// `[providers.<id>]`, by provider id. A key that is no provider's id is refused at boot.
    #[serde(flatten)]
    pub at: BTreeMap<String, ProviderUrlSettings>,
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
        // An installation token lives an hour at the provider that issues them; five minutes
        // before it expires, a new one is minted.
        Self {
            margin_seconds: 300,
        }
    }
}

/// `[providers.listing]`: how far a list that fans out over a provider's resources goes.
#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(default, deny_unknown_fields)]
pub struct ListingSettings {
    /// Resources read at once, when a list reads several (each installation of an app).
    pub concurrency: usize,
    /// Items one list answers at most.
    pub max_items: usize,
    /// The most of a downloaded text (a workflow job's logs) answered, in bytes: its end.
    pub max_log_bytes: usize,
    /// Pages read at most looking for one item a provider has no lookup for.
    pub max_scan_pages: u32,
}

impl ListingSettings {
    pub fn limits(&self) -> osiris_providers::ListLimits {
        osiris_providers::ListLimits {
            concurrency: self.concurrency,
            max_items: self.max_items,
            max_log_bytes: self.max_log_bytes,
            max_scan_pages: self.max_scan_pages,
        }
    }
}

impl Default for ListingSettings {
    fn default() -> Self {
        Self {
            concurrency: 4,
            max_items: 5000,
            // OpenConnector's cap on a job log: its last 256 KiB.
            max_log_bytes: 262_144,
            // As many as OpenConnector reads looking for a webhook it created before.
            max_scan_pages: 10,
        }
    }
}

/// `[providers.<id>]`: where one provider is, when not where it registers by default.
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct ProviderUrlSettings {
    /// Where the consent page is.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub oauth_base_url: Option<String>,
    /// Where the API is.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub api_base_url: Option<String>,
}

impl ProvidersSettings {
    /// Where `registration`'s provider is: its own defaults, with what `[providers.<id>]` sets.
    pub fn urls(&self, registration: &Registration) -> Result<ProviderUrls, url::ParseError> {
        let set = self.at.get(registration.id);
        let oauth = set.and_then(|set| set.oauth_base_url.as_deref());
        let api = set.and_then(|set| set.api_base_url.as_deref());
        Ok(ProviderUrls {
            oauth_base: oauth.unwrap_or(registration.oauth_base_url).parse()?,
            api_base: api.unwrap_or(registration.api_base_url).parse()?,
        })
    }
}
