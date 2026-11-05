//! The answer both processes give on `/health`: 200 when the database answers a cheap query, 503
//! when it doesn't, with the build's commit, so a rollout can check which revision is answering.

use axum::{Json, http::StatusCode};
use serde_json::{Value, json};

pub fn health_answer(database_up: bool) -> (StatusCode, Json<Value>) {
    let commit = osiris_env::build::commit();
    if database_up {
        (
            StatusCode::OK,
            Json(json!({ "status": "ok", "database": true, "commit": commit })),
        )
    } else {
        (
            StatusCode::SERVICE_UNAVAILABLE,
            Json(json!({ "status": "unavailable", "database": false, "commit": commit })),
        )
    }
}
