//! The current release's `OpenAPI` document, generated from the routes, with the answers every
//! `/v1` route can give before it runs (its ground rules) added to each operation.

use super::{ground_rule_answers::ground_rule_answers, v1::v1};
use serde_json::Value;

pub fn spec() -> Value {
    let mut spec = serde_json::to_value(v1().split_for_parts().1).unwrap_or_default();
    ground_rule_answers(&mut spec);
    spec
}
