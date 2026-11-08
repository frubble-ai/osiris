//! A provider whose scopes nest, for the tests of `missing_scopes`.

#![allow(
    clippy::unwrap_used,
    reason = "a mock: a failure here is the test failing"
)]

use osiris_interfaces::ProviderCommon;
use url::Url;

/// A provider whose `admin` scope covers every `admin:` one, as a provider with nesting scopes
/// says.
pub struct Nesting(Url);

impl ProviderCommon for Nesting {
    fn id(&self) -> &'static str {
        "nesting"
    }

    fn base_url(&self) -> &Url {
        &self.0
    }

    fn scope_covers(&self, granted: &str, required: &str) -> bool {
        granted == required
            || required
                .strip_prefix(granted)
                .is_some_and(|rest| rest.starts_with(':'))
    }
}

pub fn nesting() -> Nesting {
    Nesting(Url::parse("https://nesting.test").unwrap())
}
