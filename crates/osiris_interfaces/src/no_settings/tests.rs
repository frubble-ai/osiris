//! Only absent or empty objects count as no settings.

use super::no_settings;
use rstest::rstest;
use serde_json::{Value, json};

#[rstest]
#[case::empty_objects(json!({}), Some(json!({})), true)]
#[case::no_secret_settings(json!({}), None, true)]
#[case::settings_given(json!({ "app_id": "1" }), None, false)]
#[case::secret_settings_given(json!({}), Some(json!({ "key": "k" })), false)]
#[case::a_secret_array(json!({}), Some(json!([1])), false)]
#[case::a_secret_string(json!({}), Some(json!("x")), false)]
#[case::secret_null(json!({}), Some(Value::Null), false)]
#[case::settings_not_an_object(json!([]), None, false)]
fn counts_only_absent_or_empty_objects_as_no_settings(
    #[case] settings: Value,
    #[case] secret_settings: Option<Value>,
    #[case] none: bool,
) {
    assert_eq!(no_settings(&settings, secret_settings.as_ref()), none);
}
