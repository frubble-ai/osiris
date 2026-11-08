//! A version change that adds its mark to a `trail` list in whatever it translates: a request, a
//! query, an answer, an event or the spec. Three releases carry them: `2026-01-01.amun` changes
//! nothing, `2026-02-01.anubis` makes change `a`, and `2026-03-01.bastet` makes `b` then `c`.

use crate::{Release, Route, Stability, VersionChange};
use serde_json::{Value, json};

pub struct MockTrail(pub &'static str);

/// Adds `mark` to `value`'s `trail`, starting the list when there is none.
fn leave(value: &mut Value, mark: &str) {
    if let Some(object) = value.as_object_mut() {
        let trail = object.entry("trail").or_insert_with(|| json!([]));
        if let Some(trail) = trail.as_array_mut() {
            trail.push(Value::String(mark.to_owned()));
        }
    }
}

impl VersionChange for MockTrail {
    /// The change's mark.
    fn description(&self) -> &'static str {
        self.0
    }

    /// Leaves the mark in the spec.
    fn spec(&self, spec: &mut Value) {
        leave(spec, self.0);
    }

    /// Leaves the mark in a request's body.
    fn request(&self, _route: &Route<'_>, body: &mut Value) {
        leave(body, self.0);
    }

    /// Leaves the mark as a `trail` parameter.
    fn query(&self, _route: &Route<'_>, query: &mut Vec<(String, String)>) {
        query.push(("trail".to_owned(), self.0.to_owned()));
    }

    /// Leaves the mark in an answer's body.
    fn response(&self, _route: &Route<'_>, _status: u16, body: &mut Value) {
        leave(body, self.0);
    }

    /// Leaves the mark in an event.
    fn event(&self, event: &mut Value) {
        leave(event, self.0);
    }
}

/// amun, anubis (`a`) and bastet (`b`, then `c`).
pub static MOCK_RELEASES: &[Release] = &[
    Release {
        name: "2026-01-01.amun",
        number: 1,
        stability: Stability::Stable,
        changes: &[],
    },
    Release {
        name: "2026-02-01.anubis",
        number: 2,
        stability: Stability::Stable,
        changes: &[&MockTrail("a")],
    },
    Release {
        name: "2026-03-01.bastet",
        number: 3,
        stability: Stability::Preview,
        changes: &[&MockTrail("b"), &MockTrail("c")],
    },
];
