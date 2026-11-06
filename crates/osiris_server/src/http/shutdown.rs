//! Cloud Run sends SIGTERM before stopping an instance: finish in-flight work first.

pub async fn signal() {
    let ctrl_c = async {
        if let Err(error) = tokio::signal::ctrl_c().await {
            tracing::error!(%error, "listening for Ctrl-C failed");
            std::future::pending::<()>().await;
        }
    };
    let terminate = async {
        match tokio::signal::unix::signal(tokio::signal::unix::SignalKind::terminate()) {
            Ok(mut signal) => {
                signal.recv().await;
            }
            Err(error) => {
                // Without the handler only Ctrl-C can stop the server; it must not stop at once.
                tracing::error!(%error, "listening for SIGTERM failed");
                std::future::pending::<()>().await;
            }
        }
    };
    tokio::select! {
        () = ctrl_c => {},
        () = terminate => {},
    }
}
