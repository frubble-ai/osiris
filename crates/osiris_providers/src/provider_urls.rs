//! Where a provider is: its OAuth pages and its API, from `[providers.<id>]` in the settings.

use url::Url;

#[derive(Debug, Clone)]
pub struct ProviderUrls {
    /// Where the consent page is.
    pub oauth_base: Url,
    /// Where the API is; tokens are exchanged here for most providers.
    pub api_base: Url,
}
