//! The API's state: the use cases, the configuration and the bearer keys.

use crate::{
    config::{ApiConfig, Config},
    http::state::ApiState,
    use_cases::UseCases,
};
use std::sync::Arc;

pub fn state(use_cases: UseCases, config: Arc<Config>, keys: &ApiConfig) -> ApiState {
    ApiState {
        use_cases,
        config,
        api_key: Arc::new(keys.api_key.clone()),
        admin_key: Arc::new(keys.admin_key.clone()),
        releases: osiris_api_models::version::RELEASES,
    }
}
