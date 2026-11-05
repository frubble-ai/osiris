# AGENTS.md

For coding agents working in this repository. Start with README.md.

## Rules

- Rust workspace (`crates/`), axum on tokio, sqlx on Postgres. One image; `OSIRIS_PROCESS`
  (`serve`, `worker` or `migrate`) chooses the process. `serve` and `worker` only answer
  requests, so both scale to zero; work at a set time is a scheduled task (Cloud Tasks).
- The backend mirrors frubble-ai/app's, in Rust ([style guide](docs/style-guide/index.md)):
  - Everything osiris does is a use case in `use_cases/cases/`, one file each, a method on
    `UseCases` run through `run` ([use cases](docs/style-guide/use-cases.md)).
  - The context (`UseCaseContext`) is setup only, built once at boot by
    `build_use_case_context`; the configuration is read from the environment once and reaches the
    use cases as `ctx.config`.
  - `use_cases/lib/` helpers take parts of the context, never the context, and never call a use
    case.
  - Routes, the idempotency middleware and the worker call use cases and nothing else; every
    `/v1` route goes through `server_wrap`.
  - Only use cases touch storage, providers, the event bus and the task scheduler, and they reach
    the bus and the scheduler only through our traits. Vendor code lives in its own crate
    (`osiris_pubsub`, `osiris_cloud_tasks`); `build_use_case_context` is the one place that
    chooses ([third-party integrations](docs/style-guide/third-party-integrations.md)).
- Follow the [placement rule](docs/style-guide/placement-rule.md): domains mirrored in every
  layer, one unit per file, `foo.rs` + `foo/` (never `mod.rs`, and a module file only declares and
  re-exports), a helper used by one consumer next to it.
- Tests live beside the code; mocks and data factories are files in `mocks/` or `testing/`, never
  inline; test data is generated with `fake`; cases are parametrized with `rstest`
  ([test organization](docs/style-guide/test-organization.md),
  [testing patterns](docs/style-guide/testing-patterns.md)).
- Timings and limits are settings, never constants; configuration comes from the environment,
  with no file read and no environment-specific branch ([settings](docs/style-guide/settings.md)).
- Every query is a compile-time-checked `query!`/`query_as!`, in tests too. After changing one,
  `just sqlx-prepare` and commit `.sqlx/`.
- Migrations are reversible SQL in `migrations/` (`just migration <name>`), applied by the
  migrate job, never at startup.
- Never log, return or put in an error a token, a client secret or a key. They are `Secret` in
  memory and encrypted at rest with the tenant's data key. A client never reads a provider's text.
- A change to what the API answers either keeps old clients working or comes with a new version
  ([api-versioning](docs/api-versioning.md)); `just openapi` and commit the spec.
- Change the docs in the same pull request as the architecture they describe.
- `just check` must pass before a commit. Branch from `dev`; pull requests against `dev`, the
  template filled. Commit messages and PR titles are Conventional Commits.

## Docs map

- [`docs/architecture.md`](docs/architecture.md) — crates, processes, scheduled tasks, the connection and refresh flows, events, encryption.
- [`docs/configuration.md`](docs/configuration.md) — every env var, the database's TLS, process selection, settings, running locally.
- [`docs/api-versioning.md`](docs/api-versioning.md) — versions, releases, version changes.
- [`docs/add_provider.md`](docs/add_provider.md) — a new provider or action, from `provider-template/`.
- [`docs/style-guide/index.md`](docs/style-guide/index.md) — use cases, placement, integrations, tests, settings, errors, migrations.
- [`docs/changelog/`](docs/changelog/) — what each release changed for clients.
