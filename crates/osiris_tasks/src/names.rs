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
mod tests {
    use super::*;

    #[test]
    fn names_follow_what_the_task_is_for() {
        assert_eq!(refresh("ca_1", 3), refresh("ca_1", 3));
        assert_ne!(refresh("ca_1", 3), refresh("ca_1", 4));
        assert_ne!(refresh("ca_1", 3), refresh("ca_2", 3));
        assert_ne!(refresh("ca_1", 3), refresh_now("ca_1", 3));
        assert_eq!(consent("ca_1"), "consent-expire-ca_1");
        assert_eq!(event("ev_9"), "event-publish-ev_9");
    }

    #[test]
    fn names_use_only_what_schedulers_accept() {
        for name in [
            refresh("ca_Ab9", 12),
            refresh_now("ca_Ab9", 1),
            consent("ca_x"),
            event("ev_y"),
        ] {
            assert!(
                name.chars()
                    .all(|c| c.is_ascii_alphanumeric() || c == '-' || c == '_')
            );
            assert!(name.len() <= 500);
        }
    }
}
