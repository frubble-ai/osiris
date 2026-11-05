# Adding a provider or an action

A provider is one folder in `osiris_providers` and one line in each place that lists providers.
Nothing else in osiris names one.

## A provider

1. Copy [`provider-template/`](../provider-template/) into `crates/osiris_providers/src/` and
   rename `acme`/`Acme`. Fill its OAuth2 definition from the provider's docs (OpenConnector's
   definitions are a good reference), including `issues_refresh_tokens`: true when the provider
   issues refresh tokens (its connections get a scheduled refresh), false when its tokens never
   expire (nothing is scheduled). Override `parse_error` when the provider's error
   answers (non-2xx) carry their reason in its own shape. A provider that reports errors inside a
   200 (Slack's `ok: false`, Linear's GraphQL `errors`) is handled in its own transformers
   (`slack/transformers.rs`, `linear/graphql.rs`), since `call` only hands non-2xx answers to
   `parse_error`.
2. Register it in `osiris_providers/src/providers.rs`: a field in `Providers`, its argument and
   constructor call in `Providers::new`, and its entry in `Providers::get`.
3. Its URLs: `[providers.<id>]` in `ProvidersSettings` (`config/settings/providers.rs`) and
   `config.example.toml`; its entry in the provider list of `config/settings/validate.rs`; and its
   `urls()` passed to `Providers::new` in `use_cases/build_use_case_context.rs`.
4. Its row: a migration inserting it into `provider` (`just migration add_<id>`).
5. The server tests' builder: its URLs in `Providers::new` (`osiris_server/src/testing/test_context.rs`).
6. Its faked network and tests: `<id>/mocks/` (a `mock_<id>_network.rs` with an arranger per
   endpoint, a `generate_*` factory per answer type) and `<id>/tests.rs`: each action's paging, a
   rejected token reading as `Unauthorized`, a revoke of a grant already gone. Both follow
   [test organization](style-guide/test-organization.md).
7. The API models: `osiris_api_models/src/actions/<id>.rs` re-exporting the provider's types.
8. The docs: this provider in README.md and the release's changelog.

## An action

1. `<provider>/<action>.rs` with the call, `types.rs` with our answer type, `transformers.rs` with
   the provider's answer and its `From`.
2. The use case: `use_cases/cases/list_<provider>_<thing>.rs`, a method on `UseCases` that runs
   the provider call through `run_connected_account_action` (so a rejected token is refreshed once, then
   the connection revoked), declared in `use_cases/cases.rs`
   ([use cases](style-guide/use-cases.md)).
3. The route: `routes/actions/<provider>/<action>.rs` (declared in `routes/actions/<provider>.rs`),
   registered in `routes/actions/router.rs`.
4. `just openapi`, the changelog, and its tests (the provider against its faked network; the use
   case's and the route's in `services/api/tests/actions.rs`).
