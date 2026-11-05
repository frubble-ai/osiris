# Testing patterns

- **Write the test as someone who knows what the feature does, not how it is built.** If a refactor of the internals breaks the test, the test is bad. If a real bug gets through, the test is bad. A test that does not fail on a real bug is deleted.
- **Name tests from the product's point of view**: `refuses_a_key_reused_for_another_request`, not `begin_returns_mismatch`.
- **Arrange, act, assert**, visibly separated.
- **Assert behaviour a client sees**: the status, the `code`, the body's fields, the event published, the task scheduled, the outcome recorded. Not private functions, not which helper was called.

## What is tested where

- **Use cases** (`use_cases/cases/tests/`): behaviour, on a real Postgres with a fresh database per test (`#[sqlx::test(migrator = "osiris_storage::MIGRATOR")]`), the providers' network faked, and in-memory adapters to read what was published and scheduled. State is arranged through the use cases and the routes.
- **The API** (`services/api/tests/`): the contract through `api::router`: statuses, error codes, redirects, keys, versions, idempotency. Requests are real, sent through the router, not to handlers.
- **The worker** (`services/worker/tests/`): who may send a task, a body that is not a task, `/health` while draining.
- **Providers** (`osiris_providers`): each against its faked network, with the provider's real answer shapes, its paging and its error forms.
- **Adapters of our traits**: the contract, run for each implementation.
- **Storage** (`osiris_storage/tests/`): its operations on a real Postgres, including encryption and the completeness check.
- **Pure units** (parsers, mappers, state machines): unit tests beside them.
- **Live tests** against real provider test accounts are `#[ignore]` and run nightly with their tokens (`.github/workflows/live.yml`); the fakes can't tell us a provider changed. In CI a missing token fails the test.

## What a test does not do

- Check internal state, or that a library works.
- Snapshot a response.
- Poll with a sleep; assert on the outcome, or wait on the condition.
- Write rows to arrange state when a use case or route can.

## Database

Every query is `query!`/`query_as!`, in tests too ([migrations](./migrations.md)). Tests run against the same TLS-only Postgres as local development and CI, with the same `DB_*` variables ([configuration](../configuration.md)).
