"""Writes every provider's action use cases and routes, which all share one shape, from
scripts/actions/<provider>/: `provider.json` (the provider's type and display name) and one
`<group>.json` per group of its actions. It also writes each provider's group modules, its parent
modules, its routers and its list of actions (docs/providers/<provider>/actions.md). The provider call, its input and its answer's type are written by hand
in osiris_providers/src/<provider>/<group>/<action>.rs.

Each action: group, name, input (the input type), output (the answer's type), summary (what it
does, for the docs), and when they apply: list (it answers a list), no_input (it takes no body),
user_only (it only works as a user, so an installation connection is refused before the call),
acting (the provider call is told what the token acts as).

Usage, from the repository's root: python3 scripts/actions/generate.py
Then `cargo fmt`."""
import json, os

SPECS = "scripts/actions"
CASES = "crates/osiris_server/src/use_cases/cases"
ROUTES = "crates/osiris_server/src/routes/actions"
DOCS = "docs/providers"


def write(path, text):
    os.makedirs(os.path.dirname(path), exist_ok=True)
    with open(path, "w") as file:
        file.write(text)


def use_case(p, provider, a):
    g, n, ty = a["group"], a["name"], a["input"]
    out = f"Vec<{a['output']}>" if a.get("list") else a["output"]
    acting = "acting" if a.get("acting") else "_acting"
    passed = ", acting" if a.get("acting") else ""
    user_only = "true" if a.get("user_only") else "false"
    refused = "\n    /// Only as a user: an installation connection is refused before the call." if a.get("user_only") else ""
    return f'''//! `{p}.{n}`: {a["summary"]}

use crate::use_cases::{{
    UseCaseResult, UseCases,
    cases::run_connected_account_action::RunConnectedAccountActionArgs,
    lib::shared::{{action_args::ActionArgs, provider_of::provider_of}},
}};
use osiris_common::secret::{{ExposeSecret, Secret}};
use osiris_interfaces::ActingAs;
use osiris_providers::{p}::{{{provider["type"]}, {g}::{ty}, types::{a["output"]}}};

impl UseCases {{
    /// Runs `{p}.{n}` through the connection, refused first when it lacks
    /// [`{ty}::SCOPES`].{refused}
    pub async fn {p}_{n}(&self, args: ActionArgs<'_, {ty}>) -> UseCaseResult<{out}> {{
        self.run("{p}_{n}", async {{
            let ctx = self.context();
            let provider = provider_of::<{provider["type"]}>(&ctx.providers)?;
            let (http, input) = (&ctx.http, args.input);
            self.run_connected_account_action(RunConnectedAccountActionArgs {{
                id: args.id,
                provider: "{p}",
                action: "{p}.{n}",
                required_scopes: {ty}::SCOPES,
                user_only: {user_only},
                call: move |token: Secret, {acting}: ActingAs| async move {{
                    provider.{n}(http, token.expose_secret(), input{passed}).await
                }},
            }})
            .await
        }})
        .await
    }}
}}
'''


def route(p, a):
    g, n, ty = a["group"], a["name"], a["input"]
    out = f"Vec<{a['output']}>" if a.get("list") else a["output"]
    if a.get("no_input"):
        body_param, body_spec, input_expr, json_imp = "", "", f"&{ty}::default()", ""
    else:
        body_param, body_spec, input_expr, json_imp = f"\n    JsonBody(input): JsonBody<{ty}>,", f", request_body = {ty}", "&input", "\n        json_body::JsonBody,"
    return f'''//! `POST /v1/connected-accounts/{{id}}/actions/{p}.{n}`.

use crate::{{
    http::{{
        auth::Auth,
        error::ApiResult,{json_imp}
        server_wrap,
        state::ApiState,
    }},
    use_cases::lib::shared::action_args::ActionArgs,
}};
use axum::{{
    Json,
    extract::{{Path, State}},
    http::{{HeaderMap, StatusCode}},
}};
use osiris_api_models::{{
    actions::{{
        ActionResult,
        {p}::{{{g}::{ty}, {a["output"]}}},
    }},
    errors::ErrorBody,
}};
use osiris_env::Flow;

/// {a["summary"][0].upper() + a["summary"][1:]}
#[utoipa::path(post, path = "/v1/connected-accounts/{{id}}/actions/{p}.{n}", tag = "actions", params(("id" = String, Path)){body_spec},
    params(("Idempotency-Key" = Option<String>, Header, description = "Runs the request once; a retry gets the first answer")),
    responses((status = 200, body = ActionResult<{out}>), (status = 403, body = ErrorBody), (status = 404, body = ErrorBody), (status = 409, body = ErrorBody), (status = 424, body = ErrorBody), (status = 502, body = ErrorBody)),
    security(("api_key" = [])))]
pub async fn {p}_{n}(
    State(state): State<ApiState>,
    headers: HeaderMap,
    Path(id): Path<String>,{body_param}
) -> ApiResult<(StatusCode, Json<ActionResult<{out}>>)> {{
    server_wrap(
        Flow::ActionsRun,
        &state,
        &headers,
        Auth::Api,
        StatusCode::OK,
        |use_cases| async move {{
            use_cases
                .{p}_{n}(ActionArgs {{
                    id: &id,
                    input: {input_expr},
                }})
                .await
                .map(|data| ActionResult {{ data }})
        }},
    )
    .await
}}
'''


def generate(p):
    folder = f"{SPECS}/{p}"
    provider = json.load(open(f"{folder}/provider.json"))
    name = provider["name"]
    actions = [
        action
        for spec in sorted(os.listdir(folder))
        if spec.endswith(".json") and spec != "provider.json"
        for action in json.load(open(f"{folder}/{spec}"))
    ]
    cases, routes = f"{CASES}/{p}", f"{ROUTES}/{p}"
    groups = {}
    for a in actions:
        g, n = a["group"], a["name"]
        groups.setdefault(g, set()).add(n)
        write(f"{cases}/{g}/{n}.rs", use_case(p, provider, a))
        write(f"{routes}/{g}/{n}.rs", route(p, a))
    for g, names in groups.items():
        title = g.replace("_", " ")
        write(f"{cases}/{g}.rs", f"//! {name}'s actions' use cases: {title}.\n\n" + "".join(f"pub mod {n};\n" for n in sorted(names)))
        write(f"{routes}/{g}.rs", f"//! {name}'s action routes: {title}.\n\n" + "".join(f"pub mod {n};\n" for n in sorted(names | {"router"})) + "\npub use router::router;\n")
        lines = "".join(f"        .routes(routes!({n}::{p}_{n}))\n" for n in sorted(names))
        write(f"{routes}/{g}/router.rs", f"""//! The {title} action routes, every one registered.

use super::{{{", ".join(sorted(names))}}};
use crate::http::state::ApiState;
use utoipa_axum::{{router::OpenApiRouter, routes}};

pub fn router() -> OpenApiRouter<ApiState> {{
    OpenApiRouter::new()
{lines}}}
""")
    write(f"{cases}.rs", f"""//! {name}'s actions, one use case each, grouped as its API is: each runs its provider call
//! through the connection, refused first when the connection lacks the action's scopes.

""" + "".join(f"pub mod {g};\n" for g in sorted(groups))
        # A provider's own tests of its use cases, when it has them.
        + ("\n#[cfg(test)]\nmod tests;\n" if os.path.exists(f"{cases}/tests.rs") else ""))
    write(f"{routes}.rs", f"""//! {name}'s action routes, one per action, grouped as its API is.

""" + "".join(f"pub mod {g};\n" for g in sorted(set(groups) | {"router"})) + "\npub use router::router;\n")
    merges = "".join(f"        .merge({g}::router())\n" for g in sorted(groups))
    write(f"{routes}/router.rs", f"""//! {name}'s action routes: every group's router, merged.

use super::{{{", ".join(sorted(groups))}}};
use crate::http::state::ApiState;
use utoipa_axum::router::OpenApiRouter;

pub fn router() -> OpenApiRouter<ApiState> {{
    OpenApiRouter::new()
{merges}}}
""")
    rows = []
    for a in sorted(actions, key=lambda action: (action["group"], action["name"])):
        notes = []
        if a.get("user_only"):
            notes.append("as a user only")
        if a.get("acting"):
            notes.append("reads differently on an installation connection")
        rows.append(f"| `{p}.{a['name']}` | {a['group'].replace('_', ' ')} | {a['summary'][0].upper() + a['summary'][1:]} | {', '.join(notes)} |\n")
    write(f"{DOCS}/{p}/actions.md", f"""# {name}'s actions

Generated by `scripts/actions/generate.py` from `scripts/actions/{p}/`; don't edit it by hand. Each
runs at `POST /v1/connected-accounts/{{id}}/actions/<action>`; its input and answer are in the
API reference.

| Action | Group | What it does | Notes |
| --- | --- | --- | --- |
""" + "".join(rows))
    print(f"{p}: {len(actions)} actions, {len(groups)} groups")


for provider in sorted(os.listdir(SPECS)):
    if os.path.isdir(f"{SPECS}/{provider}"):
        generate(provider)
