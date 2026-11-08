//! Tests of `actions`.

use super::*;

#[test]
fn posts_to_the_connection_s_action() {
    let run = Run {
        action: "acme.list_projects".into(),
        connection: "ca_1".into(),
    };

    assert_eq!(
        run.path(),
        "/v1/connected-accounts/ca_1/actions/acme.list_projects"
    );
}
