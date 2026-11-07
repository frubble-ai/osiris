//! API versions: a date and a named release. A client sends `Osiris-Version`; every answer says the
//! version it was served at. A breaking change only comes with a new release, named after the
//! Egyptian pantheon in alphabetical order: anubis, bastet, …
//!
//! Each release is locked to the digest of its contract, so a change that would alter a released
//! contract fails the build until it comes as a new release. Each release after the first carries
//! the version changes that turn its requests, answers, events and spec back into the release
//! before, so every listed release is served at once from one current model.

pub mod contract;
pub mod contract_sha256;
pub mod header;
pub mod release;
pub mod releases;
pub mod route;
pub mod translate;
pub mod version_change;

pub use contract::contract;
pub use contract_sha256::contract_sha256;
pub use header::HEADER;
pub use release::Release;
pub use releases::{ANUBIS, CURRENT, RELEASES, find, is_known};
pub use route::Route;
pub use version_change::VersionChange;
