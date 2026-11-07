//! A version change as a real release writes one: anubis renamed an auth config's `app_id` to
//! `client_id`, so amun's requests are brought up and anubis's answers, events and spec taken back.

use osiris_api_models::version::{ANUBIS, Release, Route, VersionChange};
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

impl VersionChange for MockRenamedClientId {
    fn description(&self) -> &'static str {
        "An auth config's `app_id` is now `client_id`."
    }

    fn spec(&self, spec: &mut Value) {
        for schema in ["AuthConfig", "CreateAuthConfig", "UpdateAuthConfig"] {
            let Some(schema) = spec.pointer_mut(&format!("/components/schemas/{schema}")) else {
                continue;
            };
            if let Some(properties) = schema.get_mut("properties") {
                rename(properties, "client_id", "app_id");
            }
            if let Some(required) = schema.get_mut("required").and_then(Value::as_array_mut) {
                for name in required.iter_mut() {
                    if name == "client_id" {
                        *name = Value::String("app_id".into());
                    }
                }
            }
        }
    }

    fn request(&self, route: &Route<'_>, body: &mut Value) {
        if route.is("POST", "/v1/auth-configs") || route.is("PATCH", "/v1/auth-configs/{id}") {
            rename(body, "app_id", "client_id");
        }
    }

    fn query(&self, route: &Route<'_>, query: &mut Vec<(String, String)>) {
        if route.is("GET", "/v1/auth-configs") {
            for (name, _) in query.iter_mut() {
                if name == "app_id" {
                    "client_id".clone_into(name);
                }
            }
        }
    }

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

    fn event(&self, event: &mut Value) {
        rename(event, "connected_account_id", "connection_id");
    }
}

const AMUN: Release = Release {
    name: "2026-01-01.amun",
    contract_sha256: "",
    changes: &[],
};

const ANUBIS_RENAMING: Release = Release {
    changes: &[&MockRenamedClientId],
    ..ANUBIS
};

/// amun, then anubis carrying the rename.
pub static MOCK_RELEASES: &[Release] = &[AMUN, ANUBIS_RENAMING];
