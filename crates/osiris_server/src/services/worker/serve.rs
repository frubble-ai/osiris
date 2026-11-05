//! `OSIRIS_PROCESS=worker`: the task endpoint, listening on `PORT`. After SIGTERM, `/health`
//! answers `draining` while tasks already started finish.

use super::{router::router, state::WorkerState};
use crate::{
    config::{CoreConfig, listen::listen_address},
    http::serve_until_stopped::serve_until_stopped,
};
use std::sync::{
    Arc,
    atomic::{AtomicBool, Ordering},
};

pub async fn serve(config: CoreConfig) -> anyhow::Result<()> {
    let core = crate::services::build::core(&config).await?;
    let grace = core.settings.server.shutdown_grace();
    let draining = Arc::new(AtomicBool::new(false));
    let state = WorkerState {
        core,
        authenticator: crate::services::build::authenticator(&config)?,
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
