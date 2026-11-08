//! How a provider registers itself, from its own folder: its id, where it is by default, and how
//! it is built. `build.rs` lists every registration in [`REGISTRATIONS`].

use crate::{ListLimits, Provider, ProviderUrls};
use std::{any::Any, sync::Arc};

include!(concat!(env!("OUT_DIR"), "/registrations.rs"));

pub struct Registration {
    /// Its key, a row of the `provider` table, as [`crate::ProviderCommon::id`] answers it.
    pub id: &'static str,
    /// Where its consent page is, unless `[providers.<id>] oauth_base_url` says otherwise.
    pub oauth_base_url: &'static str,
    /// Where its API is, unless `[providers.<id>] api_base_url` says otherwise.
    pub api_base_url: &'static str,
    /// Builds it at `urls`.
    pub build: fn(&ProviderUrls, ListLimits) -> Result<Registered, url::ParseError>,
}

/// A built provider: found by id as a [`Provider`], or by its own type for its actions.
pub struct Registered {
    pub(crate) provider: Arc<dyn Provider>,
    pub(crate) typed: Arc<dyn Any + Send + Sync>,
}

impl Registered {
    pub fn new<P: Provider + 'static>(provider: P) -> Self {
        let provider = Arc::new(provider);
        Self {
            typed: provider.clone(),
            provider,
        }
    }
}
