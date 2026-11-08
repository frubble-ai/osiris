# Architecture

Osiris holds users' OAuth connections to providers, keeps their tokens fresh, and runs typed
actions with them. Each provider's own page is in [providers](providers.md). It runs as two request-driven Cloud Run services and a
migration job. All state is in Postgres.

## Crates

| Crate | Holds |
| --- | --- |
| `osiris_common` | errors (error-stack), encryption (master key, data keys, AES-256-GCM), `Secret`, ids |
| `osiris_env` | logging, request ids, the build's commit, and the `Flow` every operation is logged under |
| `osiris_domain` | auth configs, connections, tokens, events, task outcomes, pagination, `DomainError` |
| `osiris_interfaces` | `ProviderCommon` and `Provider`, the OAuth2 flow a provider's definition drives, `call` |
| `osiris_providers` | each provider in its own folder (its definition, revoke, transformers, actions and faked network), registered by `build.rs`, and a fake provider for the tests they share |
| `osiris_events` | the event catalog, the envelope, the bus (which writes each event in the pinned release, through `osiris_versioning`), the `Transport` trait, the in-memory transport |
| `osiris_pubsub` | `Transport` on Google Pub/Sub |
| `osiris_tasks` | task payloads and names, the `TaskScheduler` and `TaskAuthenticator` traits, the local and in-memory schedulers |
| `osiris_cloud_tasks` | `TaskScheduler` on Google Cloud Tasks, and the OIDC `TaskAuthenticator` |
| `osiris_versioning` | the releases osiris serves, their version changes, and the translation between them; used by `osiris_events`, `osiris_server`, `osiris_cli` and `osiris_openapi` |
| `osiris_api_models` | what the API accepts and returns; the OpenAPI spec is generated from these |
| `osiris_server` | configuration, the use cases, the HTTP layer and routes, the processes |
| `osiris_openapi` | writes `api-reference/openapi.json` (the current release) and `api-reference/versions/<release>.json` (every release served) |
| `osiris_cli` | `osiris`, the developer CLI |

## The server

A request goes **route → `server_wrap` → use case → database / providers / event bus / task
scheduler**. A route parses the request and calls one use case. `server_wrap` checks the key, opens
the flow's span, logs the outcome and maps errors.

```
osiris_server/src/
├── main.rs  services/                 # OSIRIS_PROCESS picks serve or worker; migrate runs db::migrate
├── config/  config/settings/          # read from the environment once; settings with defaults
├── http/  routes/                     # server_wrap, keys, errors, idempotency; one route per file
├── use_cases/
│   ├── build_use_case_context.rs      # builds the context once at boot
│   ├── context.rs  use_case.rs        # UseCaseContext; UseCases and run
│   ├── cases/                         # one use case per file
│   └── lib/                           # helpers by domain, including the database's operations
└── testing/                           # the test context, with its arrangers and observers
```

- `UseCaseContext` holds `config`, `db`, `messages`, `tasks`, `providers` and `http`. It is built
  before the process takes traffic and holds nothing about a request.
- `ctx.db` is the pool, the master key and the data keys. Every query is a `query!` in
  `lib/<domain>/`, an operation of `Db`. Secrets go in encrypted and come out decrypted there.
  The migrate reset's `DO` block, and the test DDL that checks which objects it drops, are
  `sqlx::query`: `query!` cannot prepare them ([migrations](style-guide/migrations.md)).
- Routes, the idempotency middleware and the worker call use cases and nothing else.
- `lib/` helpers take parts of the context and never call a use case.

### Ports and adapters

Use cases publish events through `MessageBus` (backed by the `Transport` trait) and schedule tasks
through the `TaskScheduler` trait. `build_use_case_context` builds the implementation:

| Trait | Deployed | Local development | Tests |
| --- | --- | --- | --- |
| `Transport` | Pub/Sub (`osiris_pubsub`) | the Pub/Sub emulator | in memory |
| `TaskScheduler` | Cloud Tasks (`osiris_cloud_tasks`) | timers in the process (`OSIRIS_TASKS=local`) | in memory |

Each implementation runs the same contract test ([third-party integrations](style-guide/third-party-integrations.md)).

## Processes

One image runs every process; `OSIRIS_PROCESS` chooses which.
[`.github/workflows/osiris-cd.yml`](../.github/workflows/osiris-cd.yml) builds that image and rolls
the two services onto it after the migrate job ([configuration](configuration.md#continuous-deployment)).

| `OSIRIS_PROCESS` | Runs as | Does |
| --- | --- | --- |
| `serve` | Cloud Run service, public, scales to zero | the API at `/v1` and the OAuth callback |
| `worker` | Cloud Run service, internal, scales to zero | `POST /tasks`: runs each scheduled task the task scheduler sends |
| `migrate` | Cloud Run job, once per deploy | applies the migrations, then exits, holding an advisory lock until it does, so a second execution waits. `OSIRIS_MIGRATE_RESET=true` (the [reset workflow](configuration.md#resetting-the-database)) drops this user's tables and enums in `public` first |

Configuration comes from environment variables ([configuration](configuration.md)). The database
connection is built from `DB_*` and always uses TLS, verified against `DB_CA_PEM`. Both services
answer `/health` with 503 when the database doesn't answer, and report the build's commit.

## Scheduled tasks

Work that happens at a set time is a Cloud Task: refreshing a token before it expires, failing a
consent nobody finished, and publishing again an event whose first publish failed. A task does one
job and carries what the worker needs to check that the job is still wanted. Its name is built from
that job (`osiris_tasks::names`), so scheduling the same job twice creates one task.

| Kind | Name | Scheduled | Payload |
| --- | --- | --- | --- |
| `connection.refresh` | `connection-refresh-{connection}-{generation}` | when tokens are saved | tenant, connection, token generation, reason (`expiry`, `keepalive`, `retry`) |
| `consent.expire` | `consent-expire-{connection}` | when a consent opens, due when it expires | tenant, connection |
| `event.publish` | `event-publish-{event}` | when publishing an event fails | the whole event |
| `trigger.poll` | `trigger-poll-{trigger}-{check}` | when a trigger that checks on a schedule is created, then by each check | trigger, connection, check number |

Cloud Tasks posts a due task as JSON to the worker's `/tasks` with an OIDC token for the configured
service account. The worker checks the token (Google's signature, issuer, `CLOUD_TASKS_AUDIENCE`,
`CLOUD_TASKS_SERVICE_ACCOUNT`) before it runs anything; locally it checks the worker token. The
worker answers:

- **401**: the caller could not be authenticated.
- **204**: the task is done. It ran, it was out of date, or its body is not a task.
- **500**: the queue's retry policy runs the same task again.

Every run's outcome is saved in `task_outcome`: `refreshed`, `rescheduled`, `skipped_stale`,
`failed`, `consent_expired`, `published` or `trigger_checked`. Rows older than `[tasks] outcome_retention` are deleted
a few at a time as new ones are written.

## A connection

1. `POST /v1/auth-configs` (admin key) registers a provider's app. The client secret is encrypted
   with osiris's own data key.
2. `POST /v1/connected-accounts` opens a consent: a pending connection holding a one-time OAuth
   `state` (and a PKCE verifier, for providers that use it), its `consent.expire` task, and the
   provider's consent URL. If that task cannot be scheduled, the consent is marked failed and the
   call returns an error.
3. The provider sends the browser to `/oauth/callback`. The `state` can be claimed once. Osiris
   exchanges the code for tokens, schedules the first `connection.refresh` task, and saves the
   tokens and the task's name in one write, which makes the connection `active`. It publishes
   `connection.active` and sends the browser to the client's `callback_url` with
   `connected_account_id` and `status`.
4. `POST /v1/connected-accounts/{id}/actions/{action}` runs an action with the connection's token.

### Completeness

A connection stores its tokens and the name of its pending refresh task in separate columns. Each
provider declares whether it issues refresh tokens (`issues_refresh_tokens` in its OAuth
definition):

- **Issues refresh tokens** (every provider osiris implements today): the connection is complete when it holds a
  refresh token and its refresh task. A consent that returns no refresh token, or whose first task
  cannot be scheduled, keeps no tokens and leaves the connection `disabled`.
- **Issues none** (its tokens never expire): the connection is complete with its access token, and
  nothing is scheduled.

An incomplete connection is reported with `setup_complete: false` and the status `disabled`. The
database check `connected_account_complete` requires an active row to hold an access token, and a
row that holds a refresh token to have a refresh task.

## Keeping tokens fresh

A connection with a refresh token has one pending `connection.refresh` task.

- **When it runs**: `[refresh] margin` before the access token expires. A token that does not expire
  within `[refresh] keepalive` (29 days, under Cloud Tasks' 30-day limit) gets a `keepalive` task
  after that time.
- **Generation**: every token write increases `token_generation`; a task names the generation it
  was created for.
- **Checks**: under a row lock, the worker checks that the connection exists, is active, and is on
  the task's generation. Otherwise the task ends as `skipped_stale` and nothing changes.
- **Success**: the provider returns new tokens. Osiris schedules the next task (generation + 1),
  saves the new tokens and the next task's name in one write, then answers 204. The previous task
  is cancelled unless it is the one running.
- **The next task cannot be scheduled**: the new tokens are saved, the connection becomes
  `disabled` with no task, and the worker answers 500. The retry finds a disabled connection with
  tokens and no task, schedules its refresh, and makes it `active` (outcome `rescheduled`) without
  calling the provider. An action that uses the connection does the same.
- **The provider fails**:
  - An outage or timeout (network errors, 408, 429, 5xx, or an error code that says to try again)
    keeps the connection `active`, counts the failure, and the worker answers 500.
  - A hard refusal (`invalid_grant`, a revoked grant, any other 4xx) makes the connection
    `expired`, clears its task, publishes `connection.expired`, and schedules nothing.
- **During an action**: a token that expires within the margin is refreshed first. A token the
  provider rejects (401) is refreshed once and the action runs again; if it is rejected again the
  connection becomes `expired` with `connection.revoked`.
- **Disconnect** (`DELETE /v1/connected-accounts/{id}`): the grant is ended at the provider (whenever
  osiris holds tokens), the connection is deleted, and its pending task is cancelled by name. A task
  that remains finds no connection and ends as stale.
- **Refresh now** (`POST /v1/connected-accounts/{id}/refresh`, admin key, `osiris connections
  refresh`): schedules an immediate `retry` task that replaces the scheduled one. `refresh_task`
  and `refresh_task_at` show what is scheduled.

## Events

An event says what happened and to which connection, never the connection's data. It is published
right after the change that caused it; if publishing fails, an `event.publish` task named after the
event publishes it again. If that task cannot be scheduled either, the event is lost and this is
logged. Clients ignore an event they have already received, by its `event_id`. An event is written
in the instance's pinned release (`osiris_version`), taken back through the same version changes as
answers ([API versioning](api-versioning.md)).

## Providers

Each provider is a folder of its own in every layer it reaches, and nothing outside its folders
names it except module declarations:

- `osiris_providers/src/<id>/`, with `registration.rs` (its id, default URLs and constructor).
  `build.rs` lists every folder that has one, so `Providers` finds a provider by id, or by type
  for its own actions, and `[providers.<id>]` can move its URLs.
- Its actions: `osiris_api_models/src/actions/<id>.rs`, `use_cases/cases/<id>/` and
  `routes/actions/<id>/`, generated from `scripts/actions/<id>/`. The server's `build.rs` merges
  every provider's router.
- Its tests: `services/api/tests/<id>/`, `testing/<id>.rs` and its faked network in its own
  `mocks/`. The tests every provider shares run against a fake provider
  (`osiris_providers::mocks::fake`), which only tests register.
- Its `provider` row is written from the registrations whenever the migrations run.

## Installation connections

An app's auth config can hold the app's own `settings` and `secret_settings` (encrypted like a
client secret), which its provider validates when the auth config is saved. A connection made with
an `installation_id` is `app_installation`: it holds no tokens. No user approves it, and nothing
ties an installation to the `user_id` it is made for, so only the admin key may make one; the API
key may only start a consent, which the user approves. When an action runs, the provider
mints the installation's token from the app's credentials (`Provider::installation_token`). The
token is kept in memory (`InstallationTokenCache`) until `[providers.installation_tokens]
margin_seconds` before it expires. A token the provider refuses is minted again and the call runs
once more; a second refusal makes the connection `expired`.

An action that only works as a user declares it (`user_only` in its generator entry) and is
refused on an installation connection before the provider is called. An action that reads
differently as an installation (`acting`) is told what the token acts as (`ActingAs`).

## Scopes

Each action declares the OAuth scopes it needs. Before the provider is called, the connection's
granted scopes are compared with them (`missing_scopes`, using the provider's `scope_covers` for
scopes that nest). A connection without them is refused
as `OS_11` (403). A token that reports no scopes is not checked.

## Triggers

A trigger (`tr_…`) subscribes at a connection's provider and publishes what it receives as
`trigger.fired` events. The provider validates its kind and config, creates its subscription
(`subscribe_trigger`) and removes it on delete (`unsubscribe_trigger`).

- **Deliveries**: the provider's webhook posts to `{public_url}/webhooks/triggers/{id}`,
  signed with the trigger's own random secret (HMAC-SHA256), which is encrypted at rest. The route
  is public and trusts only the signature: a delivery not signed with the secret is answered 404,
  as a missing trigger is. A signed delivery is answered 202, whether it is published or ignored.
- **Event ids**: a fired event's id is derived from the trigger and the provider's delivery id, so
  a delivery sent twice is published with the same `event_id`.
- **Checks on a schedule**: a kind that checks on a schedule records the state it starts
  from when it is created, then checks again on its own deliveries and on its `trigger.poll` task,
  every `[triggers] check_seconds`. Each check keeps a checkpoint and publishes only a change it
  has not seen. Each scheduled check is numbered and schedules the next. A check the provider
  refuses for good makes the trigger `failed`; an outage is tried again.

## Idempotency keys

A POST or DELETE with an `Idempotency-Key` runs once, and a retry gets the first answer. A key older
than `[idempotency] retention` is treated as unused. Expired keys are deleted a few at a time when a
new key is written.

## Encryption

A master key derived from `OSIRIS_ENCRYPTION_KEY` wraps one random data key per tenant: each of a
client's users, and osiris itself for client secrets. Tokens, client secrets and PKCE verifiers are
encrypted with AES-256-GCM and a fresh nonce inside the database's operations. In memory they are
`Secret`, which never prints.
