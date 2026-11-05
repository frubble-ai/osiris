//! When a refresh should actually happen.

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum When {
    /// When an action uses the token: only if it expires within the `[refresh]` margin.
    IfDue,
    /// From a refresh task, or after the provider rejected the token: whenever there is a refresh
    /// token.
    Now,
}
