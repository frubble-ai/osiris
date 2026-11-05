//! How the worker checks who sent each task, for the chosen scheduler: Google's OIDC token for
//! Cloud Tasks, the worker token for the local scheduler.

use crate::{
    config::{Config, TasksConfig},
    use_cases::build_use_case_context::http_client,
};
use osiris_cloud_tasks::{OidcAuthenticator, OidcConfig, oidc::GOOGLE_KEYS_URL};
use osiris_common::secret::ExposeSecret;
use osiris_tasks::{TaskAuthenticator, adapters::bearer::BearerAuthenticator};
use std::{sync::Arc, time::Duration};

pub fn authenticator(config: &Config) -> anyhow::Result<Arc<dyn TaskAuthenticator>> {
    Ok(match &config.tasks {
        TasksConfig::CloudTasks(cloud) => Arc::new(OidcAuthenticator::new(
            OidcConfig {
                keys_url: GOOGLE_KEYS_URL.to_owned(),
                audience: cloud.audience.clone(),
                service_account_email: cloud.service_account_email.clone(),
                keys_ttl: Duration::from_secs(config.settings.tasks.oidc_keys_ttl_seconds),
            },
            http_client(&config.settings.http)?,
        )),
        TasksConfig::Local { token, .. } => {
            Arc::new(BearerAuthenticator::new(token.expose_secret().to_owned())?)
        }
    })
}
