//! Tests of `list`.

use super::*;
use fake::{Fake, faker::internet::en::Username};

#[test]
fn filters_by_what_it_was_given_naming_the_user_as_osiris_does() {
    let user: String = Username().fake();
    let list = List {
        user: Some(user.clone()),
        provider: Some("github".into()),
        status: Some("active".into()),
        after: None,
        limit: Some(5),
    };

    assert_eq!(
        list.query(),
        [
            ("user_id", user),
            ("provider", "github".to_owned()),
            ("status", "active".to_owned()),
            ("limit", "5".to_owned()),
        ]
    );
}

#[test]
fn lists_everything_by_default() {
    let list = List {
        user: None,
        provider: None,
        status: None,
        after: None,
        limit: None,
    };

    assert!(list.query().is_empty());
}
