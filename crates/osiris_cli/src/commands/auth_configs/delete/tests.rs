//! Tests of `delete`.

use super::*;

#[test]
fn deletes_the_app_by_id() {
    assert_eq!(Delete { id: "ac_1".into() }.path(), "/v1/auth-configs/ac_1");
}
