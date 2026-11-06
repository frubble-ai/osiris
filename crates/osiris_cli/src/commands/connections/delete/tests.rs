//! Tests of `delete`.

use super::*;

#[test]
fn deletes_the_connection_by_id() {
    assert_eq!(
        Delete { id: "ca_1".into() }.path(),
        "/v1/connected-accounts/ca_1"
    );
}
