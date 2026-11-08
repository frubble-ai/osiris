//! The bus use cases send events through, writing each in the instance's pinned release
//! (`[api] pinned_version`), as Stripe writes a webhook endpoint's events in its pinned version.
//! Built once at boot by `build_use_case_context`, and by the tests' context the same way.

use anyhow::Context;
use osiris_events::{MessageBus, Transport};
use osiris_versioning::Release;
use std::sync::Arc;

/// A bus over `transport` writing events in the release of `releases` named `pinned`; one osiris
/// doesn't serve is refused (`validate` refuses it first).
pub fn build_message_bus(
    transport: Arc<dyn Transport>,
    releases: &'static [Release],
    pinned: &str,
) -> anyhow::Result<MessageBus> {
    let release = releases
        .iter()
        .find(|release| release.name == pinned)
        .with_context(|| format!("api.pinned_version {pinned:?} is not served"))?;
    Ok(MessageBus::new(transport).written_in(releases, release))
}

#[cfg(test)]
mod tests;
