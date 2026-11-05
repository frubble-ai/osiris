# Adding a provider or an action

A provider is one folder in `osiris_providers`, and one line in each place that lists providers.
Nothing else in osiris names one.

## A provider

1. Copy [`provider-template/`](../provider-template/) into `crates/osiris_providers/src/` and
   rename `acme`/`Acme`. Fill its OAuth2 definition from the provider's docs (OpenConnector's
   definitions are a good reference), including `issues_refresh_tokens`: true when the provider
   issues refresh tokens (its connections then always get a scheduled refresh), false when its
   tokens never expire (nothing is scheduled); override `parse_error` if its errors don't read by status
   (Slack answers `ok: false` beside a 200, Linear puts errors beside a 200).
2. Register it in `osiris_providers/src/lib.rs`: a field in `Providers`, its argument and
   constructor call in `Providers::new`, and its entry in `Providers::get`.
3. Its URLs: `[providers.<id>]` in `ProvidersSettings` (`config/settings/providers.rs`) and
   `config.example.toml`; its entry in the provider list of `config/settings/validate.rs`; and its
   `urls()` passed to `Providers::new` in `services/build.rs`.
4. Its row: a migration inserting it into `provider` (`just migration add_<id>`).
5. The server tests' builder: its URLs in `Providers::new` (`crates/osiris_server/tests/support/app.rs`).
6. Tests against a fake (`crates/osiris_providers/tests/<id>.rs`, wiremock): each action's
   paging, a rejected token reading as `Unauthorized`, a revoke of a grant already gone.
7. The docs: this provider in README.md and the release's changelog.

## An action

1. `<provider>/<action>.rs` with the call, `types.rs` with our answer type, `transformers.rs` with
   the provider's answer and its `From`.
2. The core: `core/actions/<provider>/<action>.rs` (declared in `core/actions/<provider>.rs`), a
   `Core::<provider>_<action>` running it through `with_token` (so a rejected token is refreshed
   once, then the connection revoked).
3. The route: `routes/actions/<provider>/<action>.rs` (declared in `routes/actions/<provider>.rs`),
   registered in `routes/actions/router.rs`.
4. `just openapi`, the changelog, and its tests (provider against a fake; the route in
   `crates/osiris_server/tests/actions.rs`).
