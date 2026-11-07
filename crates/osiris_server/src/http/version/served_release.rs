//! The release a request is served at, in its extensions for whatever runs after the version
//! middleware.

use osiris_api_models::version::Release;

#[derive(Debug, Clone, Copy)]
pub struct ServedRelease(pub &'static Release);
