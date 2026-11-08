//! Builds the use cases' context, once at boot. This is the only place that chooses the adapters
//! behind the use cases' traits: Pub/Sub for events, and Cloud Tasks or local timers for scheduled
//! tasks.

use super::{UseCaseContext, build_message_bus::build_message_bus};
use crate::use_cases::lib::database::Db;
use crate::{
    config::{Config, TasksConfig},
    db,
};
use anyhow::Context;
use osiris_cloud_tasks::CloudTasksScheduler;
use osiris_common::{crypto::MasterKey, secret::ExposeSecret};
use osiris_providers::Providers;
use osiris_pubsub::PubSubTransport;
use osiris_tasks::{
    TaskScheduler,
    adapters::local::{LocalConfig, LocalScheduler},
};
use osiris_versioning::RELEASES;
use std::{sync::Arc, time::Duration};

pub async fn build_use_case_context(config: Arc<Config>) -> anyhow::Result<UseCaseContext> {
    let settings = &config.settings;
    let master = MasterKey::from_secret(config.encryption_key.expose_secret())?;
    let providers = Providers::new(
        |registration| settings.providers.urls(registration),
        settings.providers.listing.limits(),
    )?;
    // The three connections are independent, so a cold start opens them at once.
    // Boxed: the three connections' futures together are large.
    let (pool, transport, tasks) = Box::pin(async {
        tokio::try_join!(
            db::connect(&config.database),
            async {
                PubSubTransport::connect(&config.integrations.events)
                    .await
                    .context("connecting to Pub/Sub")
            },
            scheduler(&config),
        )
    })
    .await?;
    settings.refresh.check_against(tasks.max_lead_time())?;
    settings.triggers.check_against(tasks.max_lead_time())?;
    Ok(UseCaseContext {
        db: Db::new(pool, master, settings.database.lock_timeout()),
        messages: build_message_bus(Arc::new(transport), RELEASES, &settings.api.pinned_version)?,
        tasks,
        providers: Arc::new(providers),
        http: settings.http.client()?,
        installation_tokens: Arc::default(),
        config,
    })
}

async fn scheduler(config: &Config) -> anyhow::Result<Arc<dyn TaskScheduler>> {
    Ok(match &config.integrations.tasks {
        TasksConfig::CloudTasks(cloud) => Arc::new(
            CloudTasksScheduler::connect(cloud.clone())
                .await
                .context("connecting to Cloud Tasks")?,
        ),
        TasksConfig::Local { handler_url, token } => {
            let local = &config.settings.tasks.local;
            Arc::new(LocalScheduler::new(LocalConfig {
                handler_url: handler_url.clone(),
                token: Some(token.clone()),
                retry_delay: Duration::from_secs(local.retry_delay_seconds),
                max_attempts: local.max_attempts,
            }))
        }
    })
}
