//! `OSIRIS_PROCESS=serve`: the public API, listening on `PORT` until SIGTERM.

use super::{router::router, state::state};
use crate::{
    config::{ApiConfig, listen::listen_address},
    http::{serve_until_stopped::serve_until_stopped, shutdown},
    use_cases::{UseCases, build_use_case_context::build_use_case_context},
};
use std::sync::Arc;

pub async fn serve(config: ApiConfig) -> anyhow::Result<()> {
    let app_config = Arc::new(config.config.clone());
    let grace = app_config.settings.server.shutdown_grace();
    // Built once, before taking traffic, so a broken configuration stops the process here.
    let use_cases = UseCases::new(build_use_case_context(app_config.clone()).await?);
    let router = router(state(use_cases, app_config, &config));
    let address = listen_address()?;
    let listener = tokio::net::TcpListener::bind(&address).await?;
    tracing::info!(%address, "osiris api listening");
    serve_until_stopped(listener, router, grace, shutdown::signal(), || {
        tracing::info!("stopping");
    })
    .await
}
