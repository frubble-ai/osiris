//! Tests of `get`.

use super::*;

#[test]
fn gets_the_connection_by_id() {
    assert_eq!(
        Get { id: "ca_1".into() }.path(),
        "/v1/connected-accounts/ca_1"
    );
}
