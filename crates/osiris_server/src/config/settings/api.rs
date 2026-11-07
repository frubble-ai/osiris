//! `[api]`: the API version the API key is pinned to, for requests that name none.

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(default, deny_unknown_fields)]
pub struct ApiSettings {
    pub pinned_version: String,
}

impl Default for ApiSettings {
    fn default() -> Self {
        Self {
            pinned_version: osiris_api_models::version::CURRENT.name.to_owned(),
        }
    }
}
