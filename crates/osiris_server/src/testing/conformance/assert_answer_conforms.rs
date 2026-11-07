//! An API answer, checked against its release's spec: the status is documented for the operation,
//! and a JSON body matches that answer's schema.

use super::{schema_validator::mismatches, template_of::template_of};
use crate::services::api::versioned_spec;
use axum::http::{HeaderMap, StatusCode};
use osiris_api_models::version::{HEADER, Release};
use serde_json::Value;

/// `~` and `/` escaped for a JSON pointer.
fn escape(part: &str) -> String {
    part.replace('~', "~0").replace('/', "~1")
}

pub fn assert_answer_conforms(
    releases: &[Release],
    method: &str,
    path: &str,
    status: StatusCode,
    headers: &HeaderMap,
    body: &Value,
) {
    let path = path.split('?').next().unwrap_or(path);
    if !path.starts_with("/v1/") {
        return;
    }
    // An answer refused before its release was known (an unknown version) is the current one's.
    let served = headers.get(HEADER).and_then(|value| value.to_str().ok());
    let Some(release) = served
        .and_then(|name| releases.iter().find(|release| release.name == name))
        .or_else(|| releases.last())
    else {
        return;
    };
    let spec = versioned_spec(releases, release);
    let template = template_of(&spec, path)
        .unwrap_or_else(|| panic!("{method} {path} is not a route of {release:?}'s spec"));
    let method = method.to_ascii_lowercase();
    let status = status.as_str();
    let answer = format!("/paths/{}/{method}/responses/{status}", escape(template));
    let documented = spec.pointer(&answer).unwrap_or_else(|| {
        panic!("{release:?}'s spec doesn't document {status} for {method} {template}: {body}")
    });
    let Some(schema) = documented.pointer("/content/application~1json/schema") else {
        return;
    };
    if schema.is_null() {
        return;
    }
    let found = mismatches(
        releases,
        release,
        &format!("{answer}/content/application~1json/schema"),
        body,
    );
    assert!(
        found.is_empty(),
        "{method} {template} answered {status} out of {release:?}'s spec:\n{found}\n{body}"
    );
}
