//! Keeping tokens fresh: the next refresh task for a set of tokens, saving refreshed tokens with
//! it, and restoring a connection whose refresh could not be scheduled.

pub mod cancel_previous;
pub mod check_task;
pub mod end_refused;
pub mod next_refresh;
pub mod outcome;
pub mod restore;
pub mod save_refreshed;
pub mod scheduled;
pub mod when;

pub use outcome::{Refresh, RunningTask};
pub use when::When;
