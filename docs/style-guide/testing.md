# Testing

- **Real Postgres, a fresh database per test** (`#[sqlx::test(migrator = "osiris_storage::MIGRATOR")]`),
  so locks, leases and `SKIP LOCKED` are what's tested. Locally `just up`; in CI a service.
- **Providers against a fake** (wiremock), with the provider's real answer shapes, its paging
  and its error forms. The server's tests point every provider at one fake and use the memory
  transport to read what was published.
- **Through the router**: route tests send real requests to `api::router` (keys, version,
  idempotency included), not to handlers.
- **Assert behaviour a client sees**: the status, the `code`, the body's fields, the event sent,
  the job scheduled. Not private functions.
- **Fixtures** live in `tests/support/`, one file per crate, shared with `#[path]`.
- **Live tests** against real provider test accounts are `#[ignore]` and run nightly with their
  tokens (`.github/workflows/live.yml`): the fakes can't tell us a provider changed.
