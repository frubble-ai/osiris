# Use cases

Everything osiris does is a use case. The HTTP routes, the idempotency middleware and the worker call use cases and nothing else.

1. **The context is setup, not behaviour.** `UseCaseContext` carries what is configured once at boot (`build_use_case_context`): `config`, `store`, `messages`, `tasks`, `providers` and `http`. Nothing on it runs a flow of its own, and nothing about a request is kept on it; a request reaches a use case through its arguments.
2. **Only a use case receives the context.** Anything that needs it is a use case in `use_cases/cases/`, one file each. A use case is a method on `UseCases`, the handle callers hold, and runs through `UseCases::run`, which logs its name when it is called and when it succeeds, with how long it took. A failure is returned as it is; the caller that handles it (an HTTP route, the worker) logs it once.
3. **One argument value.** A use case takes one `<Name>Args` struct, defined in its file, so a caller names every input. A use case with no input takes none.
4. **A use case may call another use case**, through `self`. Each call goes through `run` and gets its own log lines.
5. **`lib/` takes parts of the context, never the context.** A function there receives what it needs as arguments: the store, the message bus, a `Scheduling` (the task scheduler with its timeout), a settings section. It never imports `use_cases::cases` or calls a use case.
6. **Routes, the idempotency middleware and the worker call use cases**, never `lib/`, the store, a provider or a vendor client. A route parses its request, names the use case, and shapes the answer; `server_wrap` checks the key, opens the flow's span and maps the error.
7. **Callers import what they need to name a use case**: its method on `UseCases` and its `Args` from `use_cases::cases::<name>`. They may also import the types a use case returns (`lib::consent::Outcome`, `lib::refresh::Refresh`).
8. **Failures are `DomainError` reports** (`UseCaseResult`). A use case builds them with `DomainError::…` and `lib/shared/domain_errors` (`provider`, `internal`); the HTTP layer maps each to its status and code ([errors](./errors.md)).

## Layout

```
use_cases.rs                         # declares and re-exports; no logic
use_cases/
├── build_use_case_context.rs        # the clients, built once at boot
├── context.rs                       # UseCaseContext
├── use_case.rs                      # UseCases and run
├── cases.rs                         # declares each case
├── cases/                           # one use case per file
│   └── tests/                       # their behaviour, by domain
└── lib/                             # helpers on parts of the context, by domain
    ├── consent/  events/  oauth/  refresh/  shared/  tasks/
```

## Building the context

`build_use_case_context(config)` is the only place that chooses the adapters behind the use cases' traits (Pub/Sub or Cloud Tasks, or the local scheduler). It runs once, before the process takes traffic, so a broken configuration stops the process there. The configuration reaches the use cases as `ctx.config`, an `Arc<Config>` read from the environment once ([configuration](../configuration.md)); use cases pass settings sections to `lib/`, not the whole `Config`.

## Test overrides

There is no overrides parameter. A test builds its own context with `setup_test_context(pool)` (`testing/`): the real use cases over the test's database, in-memory adapters, and the providers' faked network. `setup_test_context_with(pool, |config| …)` changes the configuration fixture first. Tests arrange state through the use cases and the routes, never by writing rows.

## Adding a use case

1. `use_cases/cases/<verb_noun>.rs`: its `Args`, and one method on `UseCases` running its body through `self.run("<verb_noun>", async { … })`, with a doc comment in product terms.
2. `pub mod <verb_noun>;` in `use_cases/cases.rs`.
3. The route (or the caller) that names it.
4. Its tests in `use_cases/cases/tests/<domain>.rs`, and the route's in `services/api/tests/<domain>.rs`.
