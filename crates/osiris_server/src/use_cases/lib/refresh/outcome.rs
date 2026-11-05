//! How a refresh ended, and the task that asked for it.

use osiris_domain::ConnectedAccount;

/// The refresh task being run, when a refresh comes from one.
#[derive(Debug, Clone, Copy)]
pub struct RunningTask<'a> {
    pub name: &'a str,
    /// The token generation the task was created for.
    pub generation: i64,
}

#[derive(Debug)]
pub enum Refresh {
    /// New tokens were saved, with their next refresh task.
    Refreshed(ConnectedAccount),
    /// Nothing was needed: the token is not due, or there is no refresh token.
    Unchanged(ConnectedAccount),
    /// The task that asked was out of date: the connection is gone, no longer active, or has newer
    /// tokens.
    Stale,
    /// The connection was disabled because its last refresh could not schedule the next one. The
    /// refresh is now scheduled and the connection is active again.
    Restored(ConnectedAccount),
    /// The provider permanently refused the refresh: the connection needs reconnecting.
    Ended(ConnectedAccount),
}

impl Refresh {
    /// The connection after the refresh. `None` when the task was out of date, which only happens
    /// for a refresh started by a task.
    pub fn into_account(self) -> Option<ConnectedAccount> {
        match self {
            Self::Refreshed(account)
            | Self::Unchanged(account)
            | Self::Ended(account)
            | Self::Restored(account) => Some(account),
            Self::Stale => None,
        }
    }
}
