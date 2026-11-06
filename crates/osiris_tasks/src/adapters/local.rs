//! A scheduler for local development. Each task is a timer inside the process that scheduled it.
//! When the timer fires, the task is sent to the worker's `POST /tasks`, as Cloud Tasks would send
//! it, with the worker token. If the worker does not answer 2xx, the task is sent again after
//! `retry_delay`, up to `max_attempts` times. Do not use it in deployment: the timers are lost
//! when the process stops.

use crate::{Scheduled, Task, TaskError, TaskScheduler};
use async_trait::async_trait;
use osiris_common::secret::{ExposeSecret, Secret};
use std::{
    collections::{HashMap, HashSet},
    sync::{Arc, Mutex, PoisonError},
    time::Duration,
};
use time::OffsetDateTime;
use tokio::task::AbortHandle;

#[derive(Debug, Clone)]
pub struct LocalConfig {
    /// The worker's task endpoint, e.g. `http://localhost:8091/tasks`.
    pub handler_url: String,
    /// Sent as `Authorization: Bearer …`. The worker checks it when it is configured with one.
    pub token: Option<Secret>,
    pub retry_delay: Duration,
    pub max_attempts: u32,
}

pub struct LocalScheduler {
    config: Arc<LocalConfig>,
    http: reqwest::Client,
    timers: Arc<Mutex<HashMap<String, AbortHandle>>>,
    used: Mutex<HashSet<String>>,
}

impl LocalScheduler {
    pub fn new(config: LocalConfig) -> Self {
        Self {
            config: Arc::new(config),
            http: reqwest::Client::new(),
            timers: Arc::default(),
            used: Mutex::default(),
        }
    }

    /// Resolves when every scheduled timer has finished delivering (or been cancelled).
    #[cfg(test)]
    pub(crate) async fn wait_until_idle(&self) {
        loop {
            if self
                .timers
                .lock()
                .unwrap_or_else(PoisonError::into_inner)
                .is_empty()
            {
                return;
            }
            tokio::task::yield_now().await;
        }
    }
}

#[async_trait]
impl TaskScheduler for LocalScheduler {
    async fn schedule(&self, task: &Task) -> Result<Scheduled, TaskError> {
        if !self
            .used
            .lock()
            .unwrap_or_else(PoisonError::into_inner)
            .insert(task.name.clone())
        {
            return Ok(Scheduled::AlreadyScheduled);
        }
        let wait = (task.run_at - OffsetDateTime::now_utc())
            .try_into()
            .unwrap_or(Duration::ZERO);
        let (config, http, timers, task) = (
            self.config.clone(),
            self.http.clone(),
            self.timers.clone(),
            task.clone(),
        );
        let name = task.name.clone();
        let handle = tokio::spawn(async move {
            tokio::time::sleep(wait).await;
            deliver(&config, &http, &task).await;
            timers
                .lock()
                .unwrap_or_else(PoisonError::into_inner)
                .remove(&task.name);
        });
        self.timers
            .lock()
            .unwrap_or_else(PoisonError::into_inner)
            .insert(name, handle.abort_handle());
        Ok(Scheduled::Created)
    }

    async fn cancel(&self, name: &str) -> Result<(), TaskError> {
        if let Some(timer) = self
            .timers
            .lock()
            .unwrap_or_else(PoisonError::into_inner)
            .remove(name)
        {
            timer.abort();
        }
        Ok(())
    }
}

async fn deliver(config: &LocalConfig, http: &reqwest::Client, task: &Task) {
    for attempt in 1..=config.max_attempts {
        let mut request = http.post(&config.handler_url).json(task);
        if let Some(token) = &config.token {
            request = request.bearer_auth(token.expose_secret());
        }
        match request.send().await {
            Ok(answer) if answer.status().is_success() => return,
            Ok(answer) => {
                tracing::warn!(task = %task.name, attempt, status = %answer.status(), "the worker refused the task; trying again");
            }
            Err(error) => {
                tracing::warn!(task = %task.name, attempt, %error, "the worker could not be reached; trying again");
            }
        }
        tokio::time::sleep(config.retry_delay).await;
    }
    tracing::error!(task = %task.name, "the task was given up after its last attempt");
}
