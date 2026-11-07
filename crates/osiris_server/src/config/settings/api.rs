//! `[api]`: the API version the API key is pinned to, for requests and events that name none, and
//! how long a release stays supported once a newer one is out.

use serde::{Deserialize, Serialize};
use time::Duration;

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(default, deny_unknown_fields)]
pub struct ApiSettings {
    pub pinned_version: String,
    /// How many days a release is still served once the next one is out: the `Sunset` date its
    /// answers carry.
    pub support_days: u64,
}

impl Default for ApiSettings {
    fn default() -> Self {
        Self {
            pinned_version: osiris_api_models::version::CURRENT.name.to_owned(),
            // GitHub's window: two years from the next release.
            support_days: 730,
        }
    }
}

/// The longest support window a release can be given: a century.
pub const MAX_SUPPORT_DAYS: u64 = 36_500;

impl ApiSettings {
    /// The support window; `validate` keeps `support_days` within `MAX_SUPPORT_DAYS`, and a larger
    /// value saturates rather than overflowing.
    pub fn support(&self) -> Duration {
        i64::try_from(self.support_days)
            .ok()
            .and_then(|days| days.checked_mul(86_400))
            .map_or(Duration::MAX, Duration::seconds)
    }
}
