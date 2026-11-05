//! Every section of the settings in one value.

use super::{
    api, consent, database, http, idempotency, pagination, providers, refresh, server, tasks,
};
use serde::{Deserialize, Serialize};

/// Every section, each defaulting to the values in its own `Default`.
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(default)]
pub struct Settings {
    pub server: server::ServerSettings,
    pub api: api::ApiSettings,
    pub database: database::DatabaseSettings,
    pub http: http::HttpSettings,
    pub consent: consent::ConsentSettings,
    pub refresh: refresh::RefreshSettings,
    pub tasks: tasks::TasksSettings,
    pub idempotency: idempotency::IdempotencySettings,
    pub pagination: pagination::PaginationSettings,
    pub providers: providers::ProvidersSettings,
}
