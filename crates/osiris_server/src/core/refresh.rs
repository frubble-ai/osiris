//! Keeping tokens fresh. The main mechanism is the connection's refresh task, which runs a margin
//! before the access token expires. Refreshing when an action finds the token about to expire is
//! only a safety net. Only one refresh runs at a time per connection: the row is locked, and a
//! refresh that waited for another one sees the tokens that one saved.

pub mod cancel_previous;
pub mod new_tokens;
pub mod next_refresh;
pub mod outcome;
pub mod refresh_connection;
pub mod restore;
pub mod save_refreshed;
pub mod scheduled;
pub mod when;

pub use outcome::{Refresh, RunningTask};
pub use when::When;
