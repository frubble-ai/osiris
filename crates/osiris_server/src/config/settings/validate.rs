//! The settings checked at boot: a value that can't work stops it, naming the setting.

use super::{Settings, refresh::MAX_SCHEDULE_AHEAD_SECONDS};

impl Settings {
    /// Refuse values that can't work, naming the setting.
    pub fn validate(&self) -> anyhow::Result<()> {
        let positive = [
            (
                "server.request_timeout_seconds",
                self.server.request_timeout_seconds,
            ),
            ("server.max_body_bytes", self.server.max_body_bytes),
            (
                "server.shutdown_grace_seconds",
                self.server.shutdown_grace_seconds,
            ),
            (
                "database.max_connections",
                u64::from(self.database.max_connections),
            ),
            ("http.timeout_seconds", self.http.timeout_seconds),
            (
                "database.connect_timeout_seconds",
                self.database.connect_timeout_seconds,
            ),
            (
                "http.connect_timeout_seconds",
                self.http.connect_timeout_seconds,
            ),
            ("consent.ttl_seconds", self.consent.ttl_seconds),
            ("refresh.keepalive_seconds", self.refresh.keepalive_seconds),
            (
                "providers.listing.concurrency",
                u64::try_from(self.providers.listing.concurrency).unwrap_or(u64::MAX),
            ),
            (
                "providers.listing.max_items",
                u64::try_from(self.providers.listing.max_items).unwrap_or(u64::MAX),
            ),
            (
                "idempotency.retention_seconds",
                self.idempotency.retention_seconds,
            ),
            (
                "idempotency.purge_batch",
                u64::try_from(self.idempotency.purge_batch).unwrap_or(0),
            ),
            (
                "tasks.outcome_retention_seconds",
                self.tasks.outcome_retention_seconds,
            ),
            (
                "tasks.outcome_prune_batch",
                u64::try_from(self.tasks.outcome_prune_batch).unwrap_or(0),
            ),
            (
                "tasks.local.retry_delay_seconds",
                self.tasks.local.retry_delay_seconds,
            ),
            (
                "tasks.local.max_attempts",
                u64::from(self.tasks.local.max_attempts),
            ),
            (
                "tasks.oidc_keys_ttl_seconds",
                self.tasks.oidc_keys_ttl_seconds,
            ),
        ];
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
        anyhow::ensure!(
            self.refresh.keepalive_seconds < MAX_SCHEDULE_AHEAD_SECONDS,
            "refresh.keepalive_seconds must be under {MAX_SCHEDULE_AHEAD_SECONDS} (30 days): a task can't be scheduled further ahead"
        );
        anyhow::ensure!(
            self.refresh.margin_seconds < self.refresh.keepalive_seconds,
            "refresh.margin_seconds must be under refresh.keepalive_seconds"
        );
        Ok(())
    }
}
