//! Tests of `serve_until_stopped`.

#![allow(
    clippy::unwrap_used,
    reason = "tests: a failure here is the test failing"
)]

use super::*;
use axum::routing::get;
use std::sync::{
    Arc,
    atomic::{AtomicBool, Ordering},
};
use std::time::Instant;
use tokio::sync::oneshot;

const GRACE: Duration = Duration::from_millis(300);
const LONG_ENOUGH: Duration = Duration::from_secs(10);

async fn serving(
    router: Router,
) -> (
    std::net::SocketAddr,
    oneshot::Sender<()>,
    Arc<AtomicBool>,
    tokio::task::JoinHandle<anyhow::Result<()>>,
) {
    let listener = TcpListener::bind("127.0.0.1:0").await.unwrap();
    let address = listener.local_addr().unwrap();
    let (stop, stopped) = oneshot::channel::<()>();
    let signalled = Arc::new(AtomicBool::new(false));
    let seen = signalled.clone();
    let server = tokio::spawn(serve_until_stopped(
        listener,
        router,
        GRACE,
        async move {
            let _ = stopped.await;
        },
        move || seen.store(true, Ordering::SeqCst),
    ));
    (address, stop, signalled, server)
}

#[tokio::test]
async fn stops_at_once_when_nothing_is_running() {
    let (_, stop, signalled, server) = serving(Router::new()).await;

    stop.send(()).unwrap();
    let started = Instant::now();
    tokio::time::timeout(LONG_ENOUGH, server)
        .await
        .unwrap()
        .unwrap()
        .unwrap();

    assert!(started.elapsed() < GRACE, "it did not wait for the grace");
    assert!(signalled.load(Ordering::SeqCst));
}

#[tokio::test]
async fn stops_after_the_grace_though_a_request_never_finishes() {
    let router = Router::new().route(
        "/hang",
        get(|| async { std::future::pending::<&'static str>().await }),
    );
    let (address, stop, signalled, server) = serving(router).await;
    let request = tokio::spawn(reqwest::get(format!("http://{address}/hang")));
    tokio::time::sleep(Duration::from_millis(100)).await;

    stop.send(()).unwrap();
    let started = Instant::now();
    tokio::time::timeout(LONG_ENOUGH, server)
        .await
        .unwrap()
        .unwrap()
        .unwrap();

    assert!(started.elapsed() >= GRACE, "it waited for the grace");
    assert!(signalled.load(Ordering::SeqCst));
    request.abort();
}
