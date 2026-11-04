//! The HTTP API.

use crate::{config::Config, db};
use axum::{Json, Router, extract::State, routing::get};
use serde_json::{Value, json};
use sqlx::PgPool;
use std::time::Duration;
use tower_http::{timeout::TimeoutLayer, trace::TraceLayer};

/// What every handler reads.
#[derive(Clone)]
pub struct AppState {
    pub db: PgPool,
}

pub fn router(state: AppState) -> Router {
    Router::new()
        .route("/health", get(health))
        .layer(TraceLayer::new_for_http())
        .layer(TimeoutLayer::with_status_code(
            axum::http::StatusCode::REQUEST_TIMEOUT,
            Duration::from_secs(30),
        ))
        .with_state(state)
}

pub async fn serve(config: Config) -> anyhow::Result<()> {
    let db = db::connect(&config.database).await?;
    let address = format!("{}:{}", config.host, config.port);
    let state = AppState { db };
    let listener = tokio::net::TcpListener::bind(&address).await?;
    tracing::info!(%address, "osiris listening");
    axum::serve(listener, router(state))
        .with_graceful_shutdown(shutdown())
        .await?;
    Ok(())
}

/// Up, and the database answers.
async fn health(State(state): State<AppState>) -> Json<Value> {
    let database = sqlx::query("select 1").execute(&state.db).await.is_ok();
    Json(json!({ "status": if database { "ok" } else { "degraded" }, "database": database }))
}

/// Cloud Run sends SIGTERM before stopping an instance; finish in-flight requests first.
async fn shutdown() {
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
