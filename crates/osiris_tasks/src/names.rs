//! Task names. Each name is built from the job the task does, so scheduling the same job twice
//! produces the same name, and the scheduler refuses to create a second task with a name it
//! already has. Names contain only letters, digits, `-` and `_` (osiris ids look like `ca_Ab12`),
//! which every scheduler accepts.

/// The refresh task for a connection's tokens of generation `generation`.
pub fn refresh(connected_account_id: &str, generation: i64) -> String {
    format!("connection-refresh-{connected_account_id}-{generation}")
}

/// An immediate refresh an operator asked for, for tokens of generation `generation`.
pub fn refresh_now(connected_account_id: &str, generation: i64) -> String {
    format!("connection-refresh-{connected_account_id}-{generation}-now")
}

/// The task that ends a consent nobody finished.
pub fn consent(connected_account_id: &str) -> String {
    format!("consent-expire-{connected_account_id}")
}

/// The task that publishes an event again.
pub fn event(event_id: &str) -> String {
    format!("event-publish-{event_id}")
}

#[cfg(test)]
mod tests;
