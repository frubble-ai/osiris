//! Which connections to list.

use crate::ConnectionStatus;

#[derive(Debug, Default)]
pub struct AccountFilter {
    pub user_id: Option<String>,
    pub provider: Option<String>,
    pub auth_config_id: Option<String>,
    pub alias: Option<String>,
    pub status: Option<ConnectionStatus>,
}
