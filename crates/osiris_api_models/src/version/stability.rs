//! Whether a release's contract is still settling or settled.

use serde::Serialize;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum Stability {
    /// Its contract may still change in any way, breaking or not; clients are told so in every
    /// answer (`Osiris-Stability: preview`). Only the newest release can be a preview.
    Preview,
    /// Its contract only grows: additions are made in place, and a breaking change waits for the
    /// next release. CI holds its spec in `api-reference/versions/` to breaking nothing.
    Stable,
}

impl Stability {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Preview => "preview",
            Self::Stable => "stable",
        }
    }
}
