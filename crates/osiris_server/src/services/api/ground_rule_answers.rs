//! The answers any `/v1` operation can give before its route runs, added to the spec so each
//! documents every status it can answer: a missing or wrong key (401), an unknown version or a bad
//! `Idempotency-Key` (400), a request that takes too long (408) or a body that is too large (413),
//! an internal error (500, `OS_99`), and on a POST or DELETE, a reused or busy `Idempotency-Key`
//! (422, 409).

use serde_json::{Value, json};

pub fn ground_rule_answers(spec: &mut Value) {
    let Some(paths) = spec.get_mut("paths").and_then(Value::as_object_mut) else {
        return;
    };
    for (path, item) in paths.iter_mut() {
        if !path.starts_with("/v1/") {
            continue;
        }
        let Some(item) = item.as_object_mut() else {
            continue;
        };
        for (method, operation) in item.iter_mut() {
            let Some(responses) = operation
                .get_mut("responses")
                .and_then(Value::as_object_mut)
            else {
                continue;
            };
            let mut answers = vec![
                (
                    "400",
                    error("An unknown `Osiris-Version`, or a request that isn't valid"),
                ),
                ("401", error("A missing or wrong key")),
                (
                    "408",
                    json!({ "description": "The request took longer than the server waits" }),
                ),
                (
                    "413",
                    json!({ "description": "The body is larger than the server reads" }),
                ),
                (
                    "500",
                    error("An internal error; the request may be retried"),
                ),
            ];
            if matches!(method.as_str(), "post" | "delete") {
                answers.push((
                    "409",
                    error("The first request with this `Idempotency-Key` is still running"),
                ));
                answers.push((
                    "422",
                    error("This `Idempotency-Key` was used for another request"),
                ));
            }
            for (status, answer) in answers {
                responses.entry(status).or_insert(answer);
            }
        }
    }
}

fn error(description: &str) -> Value {
    json!({
        "description": description,
        "content": { "application/json": { "schema": { "$ref": "#/components/schemas/ErrorBody" } } },
    })
}
