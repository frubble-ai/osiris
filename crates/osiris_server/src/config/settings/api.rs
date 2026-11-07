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

impl ApiSettings {
    pub fn support(&self) -> Duration {
        Duration::days(i64::try_from(self.support_days).unwrap_or(i64::MAX))
    }
}
