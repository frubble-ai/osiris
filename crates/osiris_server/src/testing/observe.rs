//! What a test reads back: events published, task outcomes recorded, and connections as the API
//! answers them.

use super::{API_KEY, TestContext};
use osiris_events::{Message, catalog::events::EventMessage};
use serde_json::Value;

impl TestContext {
    /// The events published so far, oldest first.
    pub fn events(&self) -> Vec<EventMessage> {
        self.bus
            .published()
            .into_iter()
            .map(|(_, envelope)| {
                let Message::Event(event) = envelope.message;
                event
            })
            .collect()
    }

    /// The outcomes recorded for the connection's task runs, oldest first.
    pub async fn outcomes(&self, connected_account_id: &str) -> Vec<String> {
        sqlx::query_scalar!(
            "SELECT outcome FROM task_outcome WHERE connected_account_id = $1 ORDER BY id",
            connected_account_id
        )
        .fetch_all(&self.use_cases.context().db.pool)
        .await
        .expect("the outcomes")
    }

    /// The connection, as the API answers it.
    pub async fn account(&self, id: &str) -> Value {
        self.send(
            "GET",
            &format!("/v1/connected-accounts/{id}"),
            Some(API_KEY),
            None,
            &[],
        )
        .await
        .body
    }
}
