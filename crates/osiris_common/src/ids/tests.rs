//! Tests of `ids`.

use super::*;

#[test]
fn makes_prefixed_random_ids() {
    let first = id("ca");
    assert!(first.starts_with("ca_"));
    assert_eq!(first.len(), 27);
    assert_ne!(first, id("ca"));
    assert!(random(40).chars().all(|c| c.is_ascii_alphanumeric()));
}
