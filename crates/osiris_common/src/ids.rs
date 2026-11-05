//! Public ids: a type prefix and 24 random base62 characters (`ac_…`, `ca_…`), like Composio's.

use rand::{RngExt, distr::Alphanumeric};

/// An id with `prefix`, e.g. `id("ac")` → `ac_…`.
pub fn id(prefix: &str) -> String {
    format!("{prefix}_{}", random(24))
}

/// `length` random base62 characters, for ids and OAuth `state`.
pub fn random(length: usize) -> String {
    rand::rng()
        .sample_iter(Alphanumeric)
        .take(length)
        .map(char::from)
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn makes_prefixed_random_ids() {
        let first = id("ca");
        assert!(first.starts_with("ca_"));
        assert_eq!(first.len(), 27);
        assert_ne!(first, id("ca"));
        assert!(random(40).chars().all(|c| c.is_ascii_alphanumeric()));
    }
}
