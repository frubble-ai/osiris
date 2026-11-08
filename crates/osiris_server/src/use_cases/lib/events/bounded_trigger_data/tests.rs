//! Data that fits is kept as it is; larger data is replaced by a note of its size.

use super::bounded_trigger_data;
use serde_json::json;

#[test]
fn keeps_data_that_fits_and_omits_larger_data_naming_its_size() {
    let data = json!({ "payload": "x".repeat(100) });
    let length = serde_json::to_vec(&data).unwrap().len();

    assert_eq!(bounded_trigger_data(data.clone(), length), data);
    assert_eq!(
        bounded_trigger_data(data, length - 1),
        json!({ "omitted": true, "bytes": length })
    );
}
