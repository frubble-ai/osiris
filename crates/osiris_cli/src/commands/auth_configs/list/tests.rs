//! Tests of `list`.

use super::*;

#[test]
fn asks_for_the_page_it_was_given() {
    let list = List {
        after: Some("ac_1".into()),
        limit: Some(20),
    };

    assert_eq!(
        list.query(),
        [("after", "ac_1".to_owned()), ("limit", "20".to_owned())]
    );
}

#[test]
fn asks_for_no_page_by_default() {
    let list = List {
        after: None,
        limit: None,
    };

    assert!(list.query().is_empty());
}
