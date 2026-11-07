//! Serving an older release from the current model: the changes made since it, applied newest
//! first to take answers, events and the spec back to it, and oldest first to bring its requests up.

pub mod changes_since;
pub mod event_down;
pub mod query_up;
pub mod request_up;
pub mod response_down;
pub mod spec_down;

pub use changes_since::changes_since;
pub use event_down::event_down;
pub use query_up::query_up;
pub use request_up::request_up;
pub use response_down::response_down;
pub use spec_down::spec_down;

#[cfg(test)]
mod tests;
