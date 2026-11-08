//! The settings that must be greater than zero, by name, for `validate`.

use super::Settings;

impl Settings {
    pub(crate) fn positive_values(&self) -> Vec<(&'static str, u64)> {
        vec![
            (
                "server.request_timeout_seconds",
                self.server.request_timeout_seconds,
            ),
            ("server.max_body_bytes", self.server.max_body_bytes),
            ("server.max_answer_bytes", self.server.max_answer_bytes),
            ("api.support_days", self.api.support_days),
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
                "providers.installation_tokens.margin_seconds",
                self.providers.installation_tokens.margin_seconds,
            ),
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
                "tasks.max_concurrent",
                u64::try_from(self.tasks.max_concurrent).unwrap_or(u64::MAX),
            ),
            (
                "tasks.schedule_timeout_seconds",
                self.tasks.schedule_timeout_seconds,
            ),
            (
                "database.lock_timeout_seconds",
                self.database.lock_timeout_seconds,
            ),
            (
                "tasks.oidc_keys_ttl_seconds",
                self.tasks.oidc_keys_ttl_seconds,
            ),
            (
                "tasks.oidc_keys_refetch_seconds",
                self.tasks.oidc_keys_refetch_seconds,
            ),
        ]
    }
}
