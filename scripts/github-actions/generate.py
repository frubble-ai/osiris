"""Writes the GitHub actions' use cases and routes, which all share one shape, from actions/*.json (one
file per group), and
the group modules, the parent modules and the router that list them. The provider call, its input and its answer's
type are written by hand in osiris_providers/src/github/<group>/<action>.rs.

Each entry: group, name, input (the input type), output (the answer's type), summary (what it
does, for the docs), list (true when it answers a list), no_input (true when it takes no body).

Usage, from the repository's root: python3 scripts/github-actions/generate.py . scripts/github-actions/actions
Then `cargo fmt`."""
import json, os, sys
root = sys.argv[1]
actions = [action for name in sorted(os.listdir(sys.argv[2])) if name.endswith(".json") for action in json.load(open(os.path.join(sys.argv[2], name)))]
cases = f"{root}/crates/osiris_server/src/use_cases/cases/github"
routes = f"{root}/crates/osiris_server/src/routes/actions/github"

def camel(name): return "".join(part.title() for part in name.split("_"))

for a in actions:
    g, n = a["group"], a["name"]
    out = f"Vec<{a['output']}>" if a.get("list") else a["output"]
    ty = a["input"]
    os.makedirs(f"{cases}/{g}", exist_ok=True)
    os.makedirs(f"{routes}/{g}", exist_ok=True)
    open(f"{cases}/{g}/{n}.rs", "w").write(f'''//! `github.{n}`: {a["summary"]}

use crate::use_cases::{{
    UseCaseResult, UseCases,
    cases::{{
        github::github_action_args::GithubActionArgs,
        run_connected_account_action::RunConnectedAccountActionArgs,
    }},
}};
use osiris_common::secret::{{ExposeSecret, Secret}};
use osiris_providers::github::{{{g}::{ty}, types::{a["output"]}}};

impl UseCases {{
    /// Runs `github.{n}` through the connection, refused first when it lacks
    /// [`{ty}::SCOPES`].
    pub async fn github_{n}(
        &self,
        args: GithubActionArgs<'_, {ty}>,
    ) -> UseCaseResult<{out}> {{
        self.run("github_{n}", async {{
            let ctx = self.context();
            let (github, http, input) = (&ctx.providers.github, &ctx.http, args.input);
            self.run_connected_account_action(RunConnectedAccountActionArgs {{
                id: args.id,
                provider: "github",
                action: "github.{n}",
                required_scopes: {ty}::SCOPES,
                call: move |token: Secret| async move {{
                    github.{n}(http, token.expose_secret(), input).await
                }},
            }})
            .await
        }})
        .await
    }}
}}
''')
    if a.get("no_input"):
        body_param, body_spec, input_expr, json_imp = "", "", f"&{ty}::default()", ""
    else:
        body_param, body_spec, input_expr, json_imp = f"\n    JsonBody(input): JsonBody<{ty}>,", f", request_body = {ty}", "&input", "\n        json_body::JsonBody,"
    open(f"{routes}/{g}/{n}.rs", "w").write(f'''//! `POST /v1/connected-accounts/{{id}}/actions/github.{n}`.

use crate::{{
    http::{{
        auth::Auth,
        error::ApiResult,{json_imp}
        server_wrap,
        state::ApiState,
    }},
    use_cases::cases::github::github_action_args::GithubActionArgs,
}};
use axum::{{
    Json,
    extract::{{Path, State}},
    http::{{HeaderMap, StatusCode}},
}};
use osiris_api_models::{{
    actions::{{
        ActionResult,
        github::{{{g}::{ty}, {a["output"]}}},
    }},
    errors::ErrorBody,
}};
use osiris_env::Flow;

/// {a["summary"][0].upper() + a["summary"][1:]}
#[utoipa::path(post, path = "/v1/connected-accounts/{{id}}/actions/github.{n}", tag = "actions", params(("id" = String, Path)){body_spec},
    params(("Idempotency-Key" = Option<String>, Header, description = "Runs the request once; a retry gets the first answer")),
    responses((status = 200, body = ActionResult<{out}>), (status = 403, body = ErrorBody), (status = 404, body = ErrorBody), (status = 409, body = ErrorBody), (status = 424, body = ErrorBody), (status = 502, body = ErrorBody)),
    security(("api_key" = [])))]
pub async fn github_{n}(
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
                .github_{n}(GithubActionArgs {{
                    id: &id,
                    input: {input_expr},
                }})
                .await
                .map(|data| ActionResult {{ data }})
        }},
    )
    .await
}}
''')

# group module files, rebuilt from every action listed
groups = {}
for a in actions: groups.setdefault(a["group"], set()).add(a["name"])
for g, names in groups.items():
    open(f"{cases}/{g}.rs", "w").write(f"//! GitHub's actions' use cases: {g.replace('_', ' ')}.\n\n" + "".join(f"pub mod {n};\n" for n in sorted(names)))
    open(f"{routes}/{g}.rs", "w").write(f"//! GitHub's action routes: {g.replace('_', ' ')}.\n\n" + "".join(f"pub mod {n};\n" for n in sorted(names | {"router"})) + "\npub use router::router;\n")
    lines = "".join(f"        .routes(routes!({n}::github_{n}))\n" for n in sorted(names))
    open(f"{routes}/{g}/router.rs", "w").write(f"""//! The {g.replace('_', ' ')} action routes, every one registered.

use super::{{{", ".join(sorted(names))}}};
use crate::http::state::ApiState;
use utoipa_axum::{{router::OpenApiRouter, routes}};

pub fn router() -> OpenApiRouter<ApiState> {{
    OpenApiRouter::new()
{lines}}}
""")
# the parent modules: every group, plus the args type and the router
open(f"{cases}.rs", "w").write("""//! GitHub's actions, one use case each, grouped as GitHub's API is: each runs its provider call
//! through the connection, refused first when the connection lacks the action's scopes.

""" + "".join(f"pub mod {g};\n" for g in sorted(set(groups) | {"github_action_args"})))
open(f"{routes}.rs", "w").write("""//! GitHub's action routes, one per action, grouped as GitHub's API is.

""" + "".join(f"pub mod {g};\n" for g in sorted(set(groups) | {"router"})) + "\npub use router::router;\n")
# the routes' router: every group's
merges = "".join(f"        .merge({g}::router())\n" for g in sorted(groups))
open(f"{routes}/router.rs", "w").write(f"""//! GitHub's action routes: every group's router, merged.

use super::{{{", ".join(sorted(groups))}}};
use crate::http::state::ApiState;
use utoipa_axum::router::OpenApiRouter;

pub fn router() -> OpenApiRouter<ApiState> {{
    OpenApiRouter::new()
{merges}}}
""")
print(len(actions), "actions,", len(groups), "groups")
