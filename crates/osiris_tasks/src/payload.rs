//! The job a task does. Each kind of task has its own payload.

use crate::{ConsentTask, EventTask, RefreshTask};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "kind")]
pub enum TaskPayload {
    /// Refresh one connection's tokens, as long as they are still the tokens the task was created
    /// for.
    #[serde(rename = "connection.refresh")]
    ConnectionRefresh(RefreshTask),
    /// Mark one consent as failed if it is still open when its time runs out.
    #[serde(rename = "consent.expire")]
    ConsentExpire(ConsentTask),
    /// Publish again one event whose first publish failed.
    #[serde(rename = "event.publish")]
    EventPublish(EventTask),
}

impl TaskPayload {
    pub fn kind(&self) -> &'static str {
        match self {
            Self::ConnectionRefresh(_) => "connection.refresh",
            Self::ConsentExpire(_) => "consent.expire",
            Self::EventPublish(_) => "event.publish",
        }
    }

    /// The id of the connection the task concerns.
    pub fn connected_account_id(&self) -> &str {
        match self {
            Self::ConnectionRefresh(task) => &task.connected_account_id,
            Self::ConsentExpire(task) => &task.connected_account_id,
            Self::EventPublish(task) => &task.connected_account_id,
        }
    }
}
