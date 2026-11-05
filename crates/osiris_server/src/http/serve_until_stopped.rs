//! Serves a router until SIGTERM (or Ctrl-C). After the signal, the listener stops accepting
//! connections and requests already started may finish, for at most `grace`; then the process
//! exits anyway, so it always stops within Cloud Run's grace period.

use super::shutdown;
use axum::Router;
use std::time::Duration;
use tokio::{net::TcpListener, sync::watch};

pub async fn serve_until_stopped(
    listener: TcpListener,
    router: Router,
    grace: Duration,
    on_signal: impl FnOnce() + Send + 'static,
) -> anyhow::Result<()> {
    let (stopping, mut stopped) = watch::channel(false);
    let server = axum::serve(listener, router).with_graceful_shutdown(async move {
        shutdown::signal().await;
        on_signal();
        let _ = stopping.send(true);
    });
    let deadline = async move {
        let _ = stopped.wait_for(|stopping| *stopping).await;
        tokio::time::sleep(grace).await;
    };
    tokio::select! {
        result = server => result?,
        () = deadline => {
            tracing::warn!(grace_seconds = grace.as_secs(), "requests still running after the grace period; stopping anyway");
        }
    }
    tracing::info!("stopped");
    Ok(())
}
