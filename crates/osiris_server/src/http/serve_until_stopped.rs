//! Serves a router until `stop` completes (the process passes `shutdown::signal()`: SIGTERM or
//! Ctrl-C). After it, the listener stops accepting
//! connections and requests already started may finish, for at most `grace`; then the process
//! exits anyway, so it always stops within Cloud Run's grace period.

use axum::Router;
use std::{future::Future, time::Duration};
use tokio::{net::TcpListener, sync::watch};

pub async fn serve_until_stopped(
    listener: TcpListener,
    router: Router,
    grace: Duration,
    stop: impl Future<Output = ()> + Send + 'static,
    on_signal: impl FnOnce() + Send + 'static,
) -> anyhow::Result<()> {
    let (stopping, mut stopped) = watch::channel(false);
    let server = axum::serve(listener, router).with_graceful_shutdown(async move {
        stop.await;
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

#[cfg(test)]
mod tests;
