//! What the API's handlers read: the use cases, the configuration and the bearer keys.

use crate::{config::Config, use_cases::UseCases};
use osiris_common::secret::Secret;
use std::sync::Arc;

#[derive(Clone)]
pub struct ApiState {
    pub use_cases: UseCases,
    pub config: Arc<Config>,
    pub api_key: Arc<Secret>,
    pub admin_key: Arc<Secret>,
}
