//! Builds the core every process runs on. This is the only place that chooses the adapters behind
//! the core's interfaces: Pub/Sub for events, and Cloud Tasks or local timers for scheduled tasks.

use crate::{
    config::{CoreConfig, TasksConfig},
    core::{Core, context::http_client},
    db,
};
use anyhow::Context;
use osiris_cloud_tasks::{
    CloudTasksScheduler, OidcAuthenticator, OidcConfig, oidc::GOOGLE_KEYS_URL,
};
use osiris_common::{crypto::MasterKey, secret::ExposeSecret};
use osiris_events::MessageBus;
use osiris_providers::Providers;
use osiris_pubsub::PubSubTransport;
use osiris_storage::Store;
use osiris_tasks::{
    TaskAuthenticator, TaskScheduler,
    adapters::{
        bearer::BearerAuthenticator,
        local::{LocalConfig, LocalScheduler},
    },
};
use std::{sync::Arc, time::Duration};

pub async fn core(config: &CoreConfig) -> anyhow::Result<Core> {
    let settings = &config.settings;
    let master = MasterKey::from_secret(config.encryption_key.expose_secret())?;
    let providers = &settings.providers;
    let providers = Providers::new(
        &providers.github.urls()?,
        &providers.linear.urls()?,
        &providers.slack.urls()?,
        providers.listing.limits(),
    )?;
    // The three connections are independent, so a cold start opens them at once.
    // Boxed: the three connections' futures together are large.
    let (pool, transport, tasks) = Box::pin(async {
        tokio::try_join!(
            db::connect(&config.database),
            async {
                PubSubTransport::connect(&config.events)
                    .await
                    .context("connecting to Pub/Sub")
            },
            scheduler(config),
        )
    })
    .await?;
    if let Some(limit) = tasks.max_lead_time() {
        anyhow::ensure!(
            settings.refresh.keepalive() < time::Duration::try_from(limit)?,
            "refresh.keepalive_seconds must be under the task scheduler's limit of {} seconds",
            limit.as_secs()
        );
    }
    Ok(Core {
        store: Store::new(pool, master, settings.database.lock_timeout()),
        bus: MessageBus::new(Arc::new(transport)),
        tasks,
        providers: Arc::new(providers),
        http: http_client(&settings.http)?,
        public_url: config.public_url.clone(),
        settings: Arc::new(settings.clone()),
    })
}

async fn scheduler(config: &CoreConfig) -> anyhow::Result<Arc<dyn TaskScheduler>> {
    Ok(match &config.tasks {
        TasksConfig::CloudTasks(cloud) => Arc::new(
            CloudTasksScheduler::connect(cloud.clone())
                .await
                .context("connecting to Cloud Tasks")?,
        ),
        TasksConfig::Local { handler_url, token } => {
            let local = &config.settings.tasks.local;
            Arc::new(LocalScheduler::new(LocalConfig {
                handler_url: handler_url.clone(),
                token: Some(token.expose_secret().to_owned()),
                retry_delay: Duration::from_secs(local.retry_delay_seconds),
                max_attempts: local.max_attempts,
            }))
        }
    })
}

/// How the worker checks who sent each task, for the chosen scheduler.
pub fn authenticator(config: &CoreConfig) -> anyhow::Result<Arc<dyn TaskAuthenticator>> {
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
