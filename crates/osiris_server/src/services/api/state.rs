//! The API's state: the core and the bearer keys.

use crate::{config::ApiConfig, core::Core, http::state::ApiState};
use std::sync::Arc;

pub fn state(core: Core, config: &ApiConfig) -> ApiState {
    ApiState {
        core,
        api_key: Arc::new(config.api_key.clone()),
        admin_key: Arc::new(config.admin_key.clone()),
    }
}
