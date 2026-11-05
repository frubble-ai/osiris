//! What the worker's handlers read.

use crate::core::Core;
use osiris_tasks::TaskAuthenticator;
use std::sync::{Arc, atomic::AtomicBool};

#[derive(Clone)]
pub struct WorkerState {
    pub core: Core,
    /// Checks who sent each task: the Cloud Tasks OIDC token when deployed, the worker token
    /// locally.
    pub authenticator: Arc<dyn TaskAuthenticator>,
    /// Set once SIGTERM arrives.
    pub draining: Arc<AtomicBool>,
}
