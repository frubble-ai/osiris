# AGENTS.md

For coding agents working in this repository. Start with README.md.

## Rules

- Rust workspace (`crates/`), axum on tokio, sqlx on Postgres. One image; `OSIRIS_PROCESS`
  (`serve`, `worker` or `migrate`) chooses the process.
- Use cases ([style guide](docs/style-guide/index.md)):
  - Everything osiris does is a use case in `use_cases/cases/`, one file each, a method on
    `UseCases` run through `run` ([use cases](docs/style-guide/use-cases.md)).
  - `UseCaseContext` is setup only, built once at boot by `build_use_case_context`. The
    configuration is read from the environment once and reaches use cases as `ctx.config`.
  - `use_cases/lib/` helpers take parts of the context and never call a use case.
  - Routes, the idempotency middleware and the worker call use cases and nothing else; every `/v1`
    route goes through `server_wrap`.
  - Only use cases touch the database (`ctx.db`; every query is a `query!` in `use_cases/lib/`),
    the providers, the event bus and the task scheduler. The bus and the scheduler are reached
    through traits; vendor code is in its own crate (`osiris_pubsub`, `osiris_cloud_tasks`) and
    `build_use_case_context` builds it ([third-party
    integrations](docs/style-guide/third-party-integrations.md)).
- Follow the [placement rule](docs/style-guide/placement-rule.md): domains in every layer, one
  unit per file, `foo.rs` + `foo/` (never `mod.rs`; a module file only declares and re-exports).
- Tests live beside the code. Mocks and data factories are files in `mocks/` or `testing/`, never
  inline. Test data is generated with `fake`, and cases are parametrized with `rstest`
  ([test organization](docs/style-guide/test-organization.md),
  [testing patterns](docs/style-guide/testing-patterns.md)).
- Timings and limits are settings. Configuration comes from the environment; no file is read and no
  code branches on the environment ([settings](docs/style-guide/settings.md)).
- Every query is a compile-time-checked `query!`/`query_as!`, in tests too. A `DO` block or DDL
  is `sqlx::query`, because `query!` prepares the statement and PostgreSQL refuses `PREPARE` of
  both ([migrations](docs/style-guide/migrations.md)). After changing a checked query:
  `just sqlx-prepare`, and commit `.sqlx/`.
- Migrations are reversible SQL in `migrations/` (`just migration <name>`), applied by the migrate
  process.
- A token, a client secret or a key is never logged, returned or put in an error. They are `Secret`
  in memory and encrypted at rest with the tenant's data key. A client never reads a provider's
  text.
- A change to what the API answers or the events say keeps every stable release working, or
  waits for the next release as a version change ([api-versioning](docs/api-versioning.md) says
  what breaks). Values that grow are open strings, never closed answer enums. Run `just openapi`
  and commit the specs. Record only the preview's golden transcripts, with
  `OSIRIS_GOLDEN=record just test -E "'test(/golden::.*::<god>$/)'"`, then replay every release
  with `just test golden`.
- Docs change in the same pull request as the architecture they describe.
- `just check` passes before a commit. Branch from `dev`; pull requests target `dev` with the
  template filled. Commit messages and PR titles are Conventional Commits.

## Docs map

- [`docs/architecture.md`](docs/architecture.md) — crates, processes, scheduled tasks, the connection and refresh flows, events, encryption.
- [`docs/configuration.md`](docs/configuration.md) — every env var, the database's TLS, process selection, settings, running locally.
- [`docs/api-versioning.md`](docs/api-versioning.md) — what breaks, preview and stable releases, version changes, the checks, and how to add, break, cut or remove.
- [`docs/add_provider.md`](docs/add_provider.md) — a new provider or action, from `provider-template/`.
- [`docs/style-guide/index.md`](docs/style-guide/index.md) — use cases, placement, integrations, tests, settings, errors, migrations.
- [`docs/changelog/`](docs/changelog/) — what each release changed for clients.
