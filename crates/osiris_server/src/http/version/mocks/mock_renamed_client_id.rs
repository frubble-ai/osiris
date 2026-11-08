//! A version change as a real release writes one: anubis renamed an auth config's `app_id` to
//! `client_id` and the connections filter `label` to `alias`, and dropped two of amun's defaults (a
//! page of one auth config, a refresh's reason), so amun's requests are brought up and anubis's
//! answers, events and spec taken back.

use osiris_versioning::{ANUBIS, Release, Route, Stability, VersionChange};
use serde_json::Value;

pub struct MockRenamedClientId;

/// Moves `from` to `to` in a JSON object, if it is there.
fn rename(body: &mut Value, from: &str, to: &str) {
    if let Some(object) = body.as_object_mut()
        && let Some(value) = object.remove(from)
    {
        object.insert(to.to_owned(), value);
    }
}

/// Renames `client_id` to `app_id` in every schema that has it, inlined ones too (a list's items).
fn rename_in_schemas(value: &mut Value) {
    match value {
        Value::Object(object) => {
            if let Some(properties) = object.get_mut("properties")
                && properties.get("client_id").is_some()
            {
                rename(properties, "client_id", "app_id");
                if let Some(required) = object.get_mut("required").and_then(Value::as_array_mut) {
                    for name in required.iter_mut().filter(|name| *name == "client_id") {
                        *name = Value::String("app_id".into());
                    }
                }
            }
            object.values_mut().for_each(rename_in_schemas);
        }
        Value::Array(items) => items.iter_mut().for_each(rename_in_schemas),
        _ => {}
    }
}

impl VersionChange for MockRenamedClientId {
    /// The change as amun's clients would read it.
    fn description(&self) -> &'static str {
        "An auth config's `app_id` is now `client_id`, and the connections filter `label` is now \
         `alias`."
    }

    /// Names the fields and the filter as amun did, in every schema that has them.
    fn spec(&self, spec: &mut Value) {
        if let Some(schemas) = spec.pointer_mut("/components/schemas") {
            rename_in_schemas(schemas);
        }
        let parameters = spec
            .pointer_mut("/paths/~1v1~1connected-accounts/get/parameters")
            .and_then(Value::as_array_mut);
        for parameter in parameters.into_iter().flatten() {
            if parameter["name"] == "alias" {
                parameter["name"] = Value::String("label".into());
            }
        }
    }

    /// An amun auth config body, with `app_id` as `client_id`; amun's refresh took no body and
    /// meant `{"reason": "amun"}`.
    fn request(&self, route: &Route<'_>, body: &mut Value) {
        if route.is("POST", "/v1/auth-configs") || route.is("PATCH", "/v1/auth-configs/{id}") {
            rename(body, "app_id", "client_id");
        }
        if route.is("POST", "/v1/connected-accounts/{id}/refresh")
            && let Some(object) = body.as_object_mut()
        {
            object
                .entry("reason")
                .or_insert_with(|| Value::String("amun".into()));
        }
    }

    /// An amun connections filter, with `label` as `alias`; amun listed one auth config a page
    /// when asked for no limit.
    fn query(&self, route: &Route<'_>, query: &mut Vec<(String, String)>) {
        if route.is("GET", "/v1/auth-configs") && !query.iter().any(|(name, _)| name == "limit") {
            query.push(("limit".to_owned(), "1".to_owned()));
        }
        if route.is("GET", "/v1/connected-accounts") {
            for (name, _) in query.iter_mut() {
                if name == "label" {
                    "alias".clone_into(name);
                }
            }
        }
    }

    /// An auth config answer, one or a list, with `client_id` as `app_id`.
    fn response(&self, route: &Route<'_>, status: u16, body: &mut Value) {
        if !(200..300).contains(&status) || !route.path.starts_with("/v1/auth-configs") {
            return;
        }
        match body.get_mut("data").and_then(Value::as_array_mut) {
            Some(items) => items
                .iter_mut()
                .for_each(|item| rename(item, "client_id", "app_id")),
            None => rename(body, "client_id", "app_id"),
        }
    }

    /// An event with `connected_account_id` as amun's `connection_id`.
    fn event(&self, event: &mut Value) {
        rename(event, "connected_account_id", "connection_id");
    }
}

const AMUN: Release = Release {
    name: "2026-01-01.amun",
    number: 1,
    stability: Stability::Stable,
    changes: &[],
};

const ANUBIS_RENAMING: Release = Release {
    number: 2,
    changes: &[&MockRenamedClientId],
    ..ANUBIS
};

/// amun, then anubis carrying the rename.
pub static MOCK_RELEASES: &[Release] = &[AMUN, ANUBIS_RENAMING];
