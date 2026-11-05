//! The payload of a `connection.refresh` task: refresh one connection's tokens.

use crate::RefreshReason;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RefreshTask {
    /// The connection's user. Their data key encrypts the connection's tokens.
    pub tenant_id: String,
    pub connected_account_id: String,
    /// The token generation this task was created for. The generation goes up every time new
    /// tokens are stored. If the connection's generation has changed by the time the task runs
    /// (because the tokens were refreshed during an action, or the user reconnected), the task is
    /// out of date and does nothing.
    pub generation: i64,
    pub reason: RefreshReason,
}
