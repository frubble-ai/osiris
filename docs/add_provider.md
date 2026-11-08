# Adding a provider or an action

A provider is a folder of its own in each layer it reaches, named by its id. Nothing outside those
folders names it except module declarations (`pub mod <id>;`), and `just placement-check` fails
a build that does ([placement rule](style-guide/placement-rule.md)).

## A provider

1. Copy [`provider-template/`](../provider-template/)'s `acme.rs` and `acme/` into
   `crates/osiris_providers/src/`, and its `actions/acme/` into `scripts/actions/`, and rename
   `acme`/`Acme`. It holds every piece below: the provider, its registration, one action group
   with an input and its `SCOPES`, and the generator's specs. Fill its OAuth2 definition from the provider's docs (OpenConnector's
   definitions are a good reference), including `issues_refresh_tokens`: true when the provider
   issues refresh tokens (its connections get a scheduled refresh), false when its tokens never
   expire (nothing is scheduled). Override `parse_error` when the provider's error answers
   (non-2xx) carry their reason in its own shape. A provider that reports errors inside a 200 is
   handled in its own transformers, since `call` only hands non-2xx answers to `parse_error`.
2. Fill its `registration.rs`: its id, its default `oauth_base_url` and `api_base_url`, and how
   it is built. `build.rs` lists every folder that has one, so nothing else registers it: `Providers`
   finds it, `[providers.<id>]` can move its URLs, the tests build it at the fake network, and the
   `migrate` process writes its `provider` row.
3. Declare its module in `osiris_providers/src/lib.rs` and its API models in
   `osiris_api_models/src/actions.rs` (`pub mod <id>;`), with
   `osiris_api_models/src/actions/<id>.rs` re-exporting its action groups and types.
4. Its faked network and tests: `<id>/mocks/` (a `mock_<id>_network.rs` with an arranger per
   endpoint, a `generate_*` factory per answer type) and `<id>/tests.rs`: each action's paging, a
   rejected token reading as `Unauthorized`, a revoke of a grant already gone. Its server tests
   go in `services/api/tests/<id>/` with its arrangers in `testing/<id>.rs`. All follow
   [test organization](style-guide/test-organization.md).
5. The docs: `docs/providers/<id>.md`, and the release's changelog.

## An action

Every provider's actions share one shape, so their use cases, routes and list are generated:

1. By hand, in `osiris_providers/src/<id>/<group>/<action>.rs`: the input type, with its
   `SCOPES` and the call, and its answer type in `<id>/types/`, read from the provider's answer in
   `<id>/transformers/`.
2. Its entry in `scripts/actions/<id>/<group>.json`: `group`, `name`, `input`, `output`, `summary`,
   and when they apply `list`, `no_input`, `user_only` (it only works as a user, so an installation
   connection is refused before the call) and `acting` (the call is told what the token acts as).
   A new provider also has `scripts/actions/<id>/provider.json` with its type and display name.
3. `python3 scripts/actions/generate.py`, then `cargo fmt`. This writes
   `use_cases/cases/<id>/<group>/<action>.rs`, `routes/actions/<id>/<group>/<action>.rs`, their
   parent modules, the routers and `docs/providers/<id>/actions.md`. A new provider's use cases
   and routes are declared once in `use_cases/cases.rs` and `routes/actions.rs`; the server's
   `build.rs` merges its router.
4. `just openapi`, the changelog, and its tests: the provider against its faked network, and the
   route in `services/api/tests/<id>/`.

## A trigger kind

A provider answers for its own kinds through `Provider`'s trigger methods: `validate_trigger`,
`subscribe_trigger`, `unsubscribe_trigger`, `receive_trigger` (check the delivery's signature and
say whether it fires, is ignored or asks for a check) and, for a kind that checks on a schedule,
`trigger_checks_on_schedule` and `check_trigger`. They live in the provider's `triggers/` folder,
and its page lists each kind's config and what it delivers.
