//! Tests of `secret`.

use super::*;

#[test]
fn never_prints_its_value() {
    let token = secret("a-token");
    assert!(!format!("{token:?}").contains("a-token"));
    assert_eq!(token.expose_secret(), "a-token");
}
