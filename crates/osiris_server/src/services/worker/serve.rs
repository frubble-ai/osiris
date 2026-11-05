//! `OSIRIS_PROCESS=worker`: the task endpoint, listening on `PORT`. After SIGTERM, `/health`
//! answers `draining` while tasks already started finish.

use super::{router::router, state::WorkerState};
use crate::{
    config::{Config, listen::listen_address},
    http::serve_until_stopped::serve_until_stopped,
    use_cases::{UseCases, build_use_case_context::build_use_case_context},
};
use std::sync::{
    Arc,
    atomic::{AtomicBool, Ordering},
};

pub async fn serve(config: Config) -> anyhow::Result<()> {
    let config = Arc::new(config);
    let grace = config.settings.server.shutdown_grace();
    // Built once, before taking traffic, so a broken configuration stops the process here.
    let use_cases = UseCases::new(build_use_case_context(config.clone()).await?);
    let draining = Arc::new(AtomicBool::new(false));
    let state = WorkerState {
        use_cases,
        authenticator: super::authenticator::authenticator(&config)?,
        config,
        draining: draining.clone(),
    };
    let address = listen_address()?;
    let listener = tokio::net::TcpListener::bind(&address).await?;
    tracing::info!(%address, "osiris worker listening");
    serve_until_stopped(listener, router(state), grace, move || {
        draining.store(true, Ordering::SeqCst);
        tracing::info!("draining");
    })
    .await
}
