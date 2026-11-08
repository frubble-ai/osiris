# Use cases

Everything osiris does is a use case. The HTTP routes, the idempotency middleware and the worker call use cases and nothing else.

1. **The context is setup.** `UseCaseContext` carries what is configured once at boot (`build_use_case_context`): `config`, `db`, `messages`, `tasks`, `providers` and `http`. Nothing on it runs a flow, and nothing about a request is kept on it; a request reaches a use case through its arguments. The one exception is `installation_tokens`, the process's cache of app installations' tokens: it is built empty at boot and only ever holds short-lived tokens, so reusing one doesn't mint a new one on every call. Each process (the API, the worker) has its own; nothing else may keep state on the context.
2. **Only a use case receives the context.** A use case is a method on `UseCases`, the handle callers hold, in `use_cases/cases/`, one file each. It runs through `UseCases::run`, which logs its name when it is called and when it succeeds, with how long it took. A failure is returned as it is; the caller that handles it (an HTTP route, the worker) logs it once.
3. **One argument value.** A use case takes one `<Name>Args` struct, defined in its file. A use case with no input takes none. The one exception is a provider's action: every one takes the same two things (the connection and the action's input type), so they share `ActionArgs<'_, Input>` from `lib/shared/action_args.rs`, which the action generator (`scripts/actions/generate.py`) writes into each use case and route.
4. **A use case may call another use case**, through `self`. Each call goes through `run` and gets its own log lines.
5. **The database is `ctx.db`, and every query is in `lib/`.** `Db` holds the pool, the master key and the data keys. Its operations are `impl Db` in `lib/<domain>/`, one per file, each a compile-time-checked `query!` that encrypts and decrypts secrets. A use case calls `ctx.db.<operation>(…)`; nothing else touches the pool.
6. **`lib/` takes parts of the context, never the context.** A function there receives what it needs as arguments: the database, the message bus, a `Scheduling` (the task scheduler with its timeout), a settings section. It never imports `use_cases::cases` or calls a use case.
7. **Routes, the idempotency middleware and the worker call use cases**, not `lib/`, the database, a provider or a vendor client. A route parses its request, names the use case, and shapes the answer; `server_wrap` checks the key, opens the flow's span and maps the error.
8. **Callers import what they need to name a use case**: its method on `UseCases` and its `Args` from `use_cases::cases::<name>` (an action's route, `ActionArgs` from `lib::shared::action_args`). They may also import the types a use case returns (`lib::consent::Outcome`, `lib::refresh::Refresh`).
9. **Failures are `DomainError` reports** (`UseCaseResult`). A use case builds them with `DomainError::…` and `lib/shared/domain_errors` (`provider`, `internal`); the HTTP layer maps each to its status and code ([errors](./errors.md)).

## Layout

```
use_cases.rs                         # declares and re-exports
use_cases/
├── build_use_case_context.rs        # builds the context once at boot
├── context.rs                       # UseCaseContext
├── use_case.rs                      # UseCases and run
├── cases.rs                         # declares each case
├── cases/                           # one use case per file
│   ├── <provider>/                  # a provider's actions, generated, grouped as its API is
│   └── tests/                       # their behaviour, by domain
└── lib/                             # helpers by domain
    ├── auth_configs/  connected_accounts/  idempotency/  task_outcomes/  triggers/   # the database's operations
    ├── database/                    # Db, tenants, rows, data keys
    ├── installation_tokens/         # app installations' tokens, minted and cached in memory
    └── consent/  events/  oauth/  refresh/  shared/  tasks/
```

## Building the context

`build_use_case_context(config)` builds the adapters behind the use cases' traits (Pub/Sub or the
emulator, Cloud Tasks or the local scheduler) and runs once, before the process takes traffic; a
broken configuration stops the process there. The configuration reaches the use cases as
`ctx.config`, an `Arc<Config>` read from the environment once ([configuration](../configuration.md)).
Use cases pass settings sections to `lib/`, not the whole `Config`.

## Tests

A test builds its context with `setup_test_context(pool)` (`testing/`): the real use cases over the
test's database, in-memory adapters, and the providers' faked network.
`setup_test_context_with(pool, |config| …)` changes the configuration fixture first. Tests arrange
state through the use cases and the routes.

## Adding a use case

1. `use_cases/cases/<verb_noun>.rs`: its `Args`, and one method on `UseCases` running its body
   through `self.run("<verb_noun>", async { … })`, with a doc comment in product terms.
2. `pub mod <verb_noun>;` in `use_cases/cases.rs`.
3. The route (or other caller) that names it.
4. Its tests in `use_cases/cases/tests/<domain>.rs`, and the route's in
   `services/api/tests/<domain>.rs`.
