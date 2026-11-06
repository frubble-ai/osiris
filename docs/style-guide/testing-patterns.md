# Testing patterns

- **A test describes what the feature does, not how it is built.** A refactor of the internals does not break it, and a real bug does. A test that does not fail on a real bug is deleted.
- **Name tests from the product's point of view**: `refuses_a_key_reused_for_another_request`, not `begin_returns_mismatch`.
- **Arrange, act, assert**, visibly separated.
- **Assert behaviour a client sees**: the status, the `code`, the body's fields, the event published, the task scheduled, the outcome recorded. Not private functions, not which helper was called.

## What is tested where

- **Use cases** (`use_cases/cases/tests/`): behaviour, on a real Postgres with a fresh database per test (`#[sqlx::test(migrator = "crate::db::MIGRATOR")]`), the providers' network faked, and in-memory adapters to read what was published and scheduled. State is arranged through the use cases and the routes.
- **The API** (`services/api/tests/`): the contract through `api::router`: statuses, error codes, redirects, keys, versions, idempotency. Requests are real, sent through the router, not to handlers.
- **The worker** (`services/worker/tests/`): who may send a task, a body that is not a task, `/health` while draining.
- **Providers** (`osiris_providers`): each against its faked network, with the provider's real answer shapes, its paging and its error forms.
- **Adapters of our traits**: the contract, run for each implementation.
- **The database's operations** (`use_cases/lib/<domain>/tests.rs`): on a real Postgres, including encryption and the completeness check.
- **Pure units** (parsers, mappers, state machines): unit tests beside them.
- **Live tests** run against real provider test accounts. They are `#[ignore]`, run nightly with their tokens (`.github/workflows/live.yml`), and fail in CI when a token is missing.

## What a test avoids

- Internal state, and whether a library works.
- Snapshots of a response.
- Sleeping to wait; it asserts on the outcome or waits on the condition.
- Writing rows to arrange state when a use case or route can.

## Database

Every query is `query!`/`query_as!`, in tests too ([migrations](./migrations.md)). A `DO` block and DDL are the exception recorded there, because `query!` cannot prepare them. Tests run against the TLS-only Postgres of local development and CI, with the same `DB_*` variables ([configuration](../configuration.md)).
