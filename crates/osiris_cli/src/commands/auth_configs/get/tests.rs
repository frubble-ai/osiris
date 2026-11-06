//! Tests of `get`.

use super::*;

#[test]
fn gets_the_app_by_id() {
    assert_eq!(Get { id: "ac_1".into() }.path(), "/v1/auth-configs/ac_1");
}
