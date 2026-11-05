//! Cloud Run sends SIGTERM before stopping an instance: finish in-flight work first.

pub async fn signal() {
    let ctrl_c = async {
        let _ = tokio::signal::ctrl_c().await;
    };
    let terminate = async {
        if let Ok(mut signal) =
            tokio::signal::unix::signal(tokio::signal::unix::SignalKind::terminate())
        {
            signal.recv().await;
        }
    };
    tokio::select! {
        () = ctrl_c => {},
        () = terminate => {},
    }
}
