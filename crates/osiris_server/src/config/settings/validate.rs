//! The settings checked at boot: a value that can't work stops it, naming the setting.

use super::Settings;

impl Settings {
    /// Refuse values that can't work, naming the setting.
    pub fn validate(&self) -> anyhow::Result<()> {
        let positive = self.positive_values();
        for (name, value) in positive {
            anyhow::ensure!(value > 0, "{name} must be positive");
        }
        anyhow::ensure!(
            0 < self.pagination.default_limit
                && self.pagination.default_limit <= self.pagination.max_limit,
            "pagination.default_limit must be between 1 and pagination.max_limit"
        );
        anyhow::ensure!(
            osiris_api_models::version::is_known(&self.api.pinned_version),
            "api.pinned_version {:?} is not a version osiris serves",
            self.api.pinned_version
        );
        for (name, urls) in [
            ("github", &self.providers.github),
            ("linear", &self.providers.linear),
            ("slack", &self.providers.slack),
        ] {
            urls.urls()
                .map_err(|error| anyhow::anyhow!("providers.{name}: {error}"))?;
        }
        anyhow::ensure!(
            self.consent.state_length >= 32,
            "consent.state_length must be at least 32"
        );
        // The worker's concurrency limit is a Tokio semaphore, which panics on more permits.
        anyhow::ensure!(
            self.tasks.max_concurrent <= tokio::sync::Semaphore::MAX_PERMITS,
            "tasks.max_concurrent must be at most {}",
            tokio::sync::Semaphore::MAX_PERMITS
        );
        anyhow::ensure!(
            self.refresh.margin_seconds < self.refresh.keepalive_seconds,
            "refresh.margin_seconds must be under refresh.keepalive_seconds"
        );
        Ok(())
    }
}

#[cfg(test)]
mod tests;
