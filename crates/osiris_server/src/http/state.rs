//! What the API's handlers read.

use crate::core::Core;
use osiris_common::secret::Secret;
use std::sync::Arc;

#[derive(Clone)]
pub struct ApiState {
    pub core: Core,
    pub api_key: Arc<Secret>,
    pub admin_key: Arc<Secret>,
}
