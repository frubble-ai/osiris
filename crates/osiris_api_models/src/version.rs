//! API versions: a date and a named release. A client sends `Osiris-Version`; every answer says the
//! version it was served at. A breaking change only comes with a new release, named after the
//! Egyptian pantheon in alphabetical order: anubis, bastet, …
//!
//! A release is a preview, whose contract may still change in any way, or stable, whose contract
//! only grows: CI holds every stable release's spec to breaking nothing (docs/api-versioning.md
//! says what breaks). Each release after the first carries the version changes that turn its
//! requests, answers, events and spec back into the release before, so every listed release is
//! served at once from one current model.

pub mod header;
pub mod release;
pub mod releases;
pub mod route;
pub mod stability;
pub mod translate;
pub mod version_change;

pub use header::{HEADER, STABILITY_HEADER};
pub use release::Release;
pub use releases::{ANUBIS, CURRENT, RELEASES, find, is_known};
pub use route::Route;
pub use stability::Stability;
pub use version_change::VersionChange;
