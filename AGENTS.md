# AGENTS.md

For coding agents working in this repository. Start with README.md.

## Rules

- Rust workspace (`crates/`), axum on tokio, sqlx on Postgres. One image; `OSIRIS_PROCESS`
  (`serve`, `worker` or `migrate`) chooses the process. `serve` and `worker` only answer requests, so
  both scale to zero; work at a set time is a scheduled task (Cloud Tasks).
- Follow the [style guide](docs/style-guide/index.md): every layer split by domain, one unit
  per file (`foo.rs` + `foo/`, never `mod.rs`), timings and limits in settings, never constants.
- Every `/v1` route goes through `server_wrap`; routes parse and call the core, nothing else.
  Only the core touches storage, providers, the event bus and the task scheduler, and it reaches
  the bus and the scheduler only through their traits. Google Cloud code lives in its own crates
  (`osiris_pubsub`, `osiris_cloud_tasks`), chosen in `services/build.rs`.
- Every query is a compile-time-checked `query!`/`query_as!`. After changing one, `just
  sqlx-prepare` and commit `.sqlx/`.
- Migrations are reversible SQL in `migrations/` (`just migration <name>`), applied by the
  migrate job, never at startup.
- Never log, return or put in an error a token, a client secret or a key. They are `Secret` in
  memory and encrypted at rest with the tenant's data key.
- A change to what the API answers either keeps old clients working or comes with a new version
  ([api-versioning](docs/api-versioning.md)); `just openapi` and commit the spec.
- `just check` must pass before a commit. Branch from `dev`; pull requests against `dev`, the
  template filled. Commit messages and PR titles are Conventional Commits.

## Docs map

- [`docs/architecture.md`](docs/architecture.md) — crates, processes, scheduled tasks, the connection and refresh flows, events, encryption.
- [`docs/configuration.md`](docs/configuration.md) — every env var, the database's TLS, process selection, settings, running locally.
- [`docs/api-versioning.md`](docs/api-versioning.md) — versions, releases, version changes.
- [`docs/add_provider.md`](docs/add_provider.md) — a new provider or action, from `provider-template/`.
- [`docs/style-guide/index.md`](docs/style-guide/index.md) — placement, errors, testing, migrations.
- [`docs/changelog/`](docs/changelog/) — what each release changed for clients.
