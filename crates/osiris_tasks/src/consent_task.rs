//! The payload of a `consent.expire` task: mark a consent as failed if nobody finished it in time.

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ConsentTask {
    pub tenant_id: String,
    pub connected_account_id: String,
}
