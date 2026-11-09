//! A provider's real URLs, as its settings name them.

use osiris_providers::ProviderUrls;
use url::Url;

pub fn urls(oauth: &str, api: &str) -> ProviderUrls {
    ProviderUrls {
        oauth_base: Url::parse(oauth).unwrap(),
        api_base: Url::parse(api).unwrap(),
    }
}
