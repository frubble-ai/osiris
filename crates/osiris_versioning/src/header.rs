//! The header a request names its version in, and every answer names the version it was served at.

pub const HEADER: &str = "osiris-version";

/// Every answer's release's stability: `preview` or `stable`.
pub const STABILITY_HEADER: &str = "osiris-stability";
