//! `OSIRIS_PROCESS=serve`: the public API, listening on `PORT` until SIGTERM.

use super::{router::router, state::state};
use crate::{
    config::{ApiConfig, listen::listen_address},
    http::serve_until_stopped::serve_until_stopped,
};

pub async fn serve(config: ApiConfig) -> anyhow::Result<()> {
    let core = crate::services::build::core(&config.core).await?;
    let grace = core.settings.server.shutdown_grace();
    let router = router(state(core, &config));
    let address = listen_address()?;
    let listener = tokio::net::TcpListener::bind(&address).await?;
    tracing::info!(%address, "osiris api listening");
    serve_until_stopped(listener, router, grace, || tracing::info!("stopping")).await
}
