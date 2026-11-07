//! `Osiris-Version`: a request names the release it was written for; one osiris doesn't serve is
//! refused, and one naming none is served at the API key's pinned version (`[api]`). An older
//! release is served from the current model: its request is brought up and the answer taken back
//! through the version changes made since. Every answer names the release it was served at, and
//! one served at a superseded release carries `Deprecation` and `Sunset`.

pub mod lifecycle_headers;
pub mod resolve_release;
pub mod serve_version;
pub mod superseded;
pub mod translate_request;
pub mod translate_response;

pub use serve_version::serve_version;

#[cfg(test)]
pub mod mocks;
#[cfg(test)]
mod tests;
