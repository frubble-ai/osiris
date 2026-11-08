//! What the API's handlers read: the use cases, the configuration, the bearer keys, and the
//! releases served.

use crate::{config::Config, use_cases::UseCases};
use osiris_common::secret::Secret;
use osiris_versioning::Release;
use std::sync::Arc;

#[derive(Clone)]
pub struct ApiState {
    pub use_cases: UseCases,
    pub config: Arc<Config>,
    pub api_key: Arc<Secret>,
    pub admin_key: Arc<Secret>,
    /// Every release served, oldest first; `RELEASES`, except in tests of the version changes.
    pub releases: &'static [Release],
}
