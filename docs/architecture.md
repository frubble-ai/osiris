# Architecture

Osiris runs entirely on request-driven services: nothing is always on. `serve` answers the
public API and the OAuth callback. `worker` answers the task scheduler. Both scale to zero when
there is nothing to do. All state lives in Postgres.

## Crates

One workspace, with one crate per layer. A crate depends only on crates above it in this table.

| Crate | Holds |
| --- | --- |
| `osiris_common` | errors (error-stack, `ErrorSwitch`), encryption (master key, data keys, AES-256-GCM), `Secret`, ids |
| `osiris_env` | logging, request ids, and the `Flow` every operation is logged under |
| `osiris_domain` | osiris's own types: auth configs, connections, tokens, events, task outcomes, `DomainError` |
| `osiris_interfaces` | `ProviderCommon` and `Provider`, the OAuth2 flow that any provider's definition drives, `call` |
| `osiris_providers` | GitHub, Linear and Slack: each one's definition, revoke, transformers and actions |
| `osiris_events` | events for clients: the catalog, the envelope, the bus, the `Transport` trait, and an in-memory transport |
| `osiris_pubsub` | the `Transport` implementation for Google Pub/Sub |
| `osiris_tasks` | scheduled tasks: their payloads and names, the `TaskScheduler` trait, and the local and in-memory schedulers |
| `osiris_cloud_tasks` | the `TaskScheduler` implementation for Google Cloud Tasks |
| `osiris_storage` | Postgres: one module per table, one file per operation; encryption happens inside |
| `osiris_api_models` | what the API accepts and returns; the OpenAPI spec is generated from these |
| `osiris_server` | config, the use cases, the HTTP layer and routes, and the processes |
| `osiris_openapi` | writes `api-reference/openapi.json` |
| `osiris_cli` | `osiris`, the developer CLI |

In `osiris_server`, a request goes **route → `server_wrap` → use case → storage / providers / bus /
task scheduler**. A route parses the request and calls one use case. `server_wrap` checks the key,
opens the flow's span, logs the outcome and maps errors. A use case is the only code that reaches
the outside world, and it is the same pattern as the app's backend ([use cases](style-guide/use-cases.md)):

```
osiris_server/src/
├── main.rs  services/                 # OSIRIS_PROCESS picks serve or worker; migrate runs db::migrate
├── config/  config/settings/          # read from the environment once; settings with defaults
├── http/  routes/                     # server_wrap, keys, errors, idempotency; one route per file
├── use_cases/
│   ├── build_use_case_context.rs      # the clients, built once at boot; chooses the adapters
│   ├── context.rs  use_case.rs        # UseCaseContext (setup only); UseCases and run
│   ├── cases/                         # one use case per file
│   └── lib/                           # helpers on parts of the context, by domain
└── testing/                           # the test context, with its arrangers and observers
```

- **The context is setup only**: `config`, `store`, `messages`, `tasks`, `providers`, `http`. It
  is built once, before the process takes traffic. Nothing about a request is on it.
- **Callers are thin**: routes, the idempotency middleware and the worker call use cases and nothing
  else.
- **`lib/` takes parts of the context** and never calls a use case.

### Ports and adapters

A use case never depends on Google Cloud. It publishes events through `MessageBus` (backed by the
`Transport` trait) and schedules tasks through the `TaskScheduler` trait. The implementation behind
each trait is chosen in one place, `build_use_case_context`, when a process starts:

| Trait | Deployed | Local development | Tests |
| --- | --- | --- | --- |
| `Transport` | Pub/Sub (`osiris_pubsub`) | the Pub/Sub emulator, same code | in memory |
| `TaskScheduler` | Cloud Tasks (`osiris_cloud_tasks`) | timers in the process (`OSIRIS_TASKS=local`) | in memory |

This mirrors how the app keeps Composio behind its broker interface. Replacing Pub/Sub or Cloud
Tasks means writing a new implementation of the trait, which runs the same contract test, without
changing a use case ([third-party integrations](style-guide/third-party-integrations.md)).

## Processes

One image runs every process; `OSIRIS_PROCESS` chooses which (it is required, with no default),
so the platform only sets environment variables:

| `OSIRIS_PROCESS` | Runs as | Does |
| --- | --- | --- |
| `serve` | Cloud Run service, public, scales to zero | the API at `/v1` and the OAuth callback |
| `worker` | Cloud Run service, internal, scales to zero | `POST /tasks`: runs each scheduled task when the task scheduler sends it |
| `migrate` | Cloud Run job, once per deploy | applies the migrations, then exits |

There is no scheduler process, no polling and no leader election. Work that must happen at a set
time is a Cloud Task.

All configuration comes from environment variables, the same way in every environment, local
included ([configuration](configuration.md)). The database connection is built from the
platform's `DB_*` variables and always uses TLS, verified against `DB_CA_PEM`. Both services
answer `/health` with 503 when the database doesn't answer, and report the build's commit.

## Scheduled tasks

Some work must happen at a set time, when no request is arriving: refreshing a token before it
expires, failing a consent nobody finished, and publishing again an event whose first publish
failed. Each of these is one Cloud Task.

Every task does one job and carries what the worker needs to check that the job is still wanted.
Its name is built from that job (see `osiris_tasks::names`), so scheduling the same job twice
creates only one task:

| Kind | Name | Scheduled | Payload |
| --- | --- | --- | --- |
| `connection.refresh` | `connection-refresh-{connection}-{generation}` | when tokens are saved | tenant, connection, token generation, reason (`expiry`, `keepalive`, `retry`) |
| `consent.expire` | `consent-expire-{connection}` | when a consent opens, due when it expires | tenant, connection |
| `event.publish` | `event-publish-{event}` | when publishing an event fails | the whole event |

When a task is due, Cloud Tasks posts it as JSON to the worker's `/tasks` with an OIDC token for
the configured service account. Cloud Run checks that token before the worker sees the request.
The worker's answer decides what happens next:

- **204**: the task is done. It ran, it was out of date, or it can never run (a body that is not
  a task).
- **500**: the queue's retry policy runs the same task again later.

Every run's outcome is saved in `task_outcome`: `refreshed`, `rescheduled`, `skipped_stale`, `failed`,
`consent_expired` or `published`. Rows older than `[tasks] outcome_retention` are deleted a few
at a time as new ones are written.

## A connection

1. The client registers its provider app once: `POST /v1/auth-configs` (admin key). The client
   secret is encrypted with osiris's own data key.
2. `POST /v1/connected-accounts` opens a consent. It creates a pending connection holding a
   one-time OAuth `state` (and a PKCE verifier, for providers that use it), schedules the
   consent's `consent.expire` task, and returns the provider's consent URL. If that task cannot be
   scheduled, the consent is marked failed and the call returns an error.
3. The provider sends the browser to `/oauth/callback`. The `state` can be claimed only once.
   Osiris exchanges the code for tokens, schedules the first `connection.refresh` task, and then
   saves the tokens and the task's name in one write, which makes the connection `active`. It
   publishes `connection.active` and sends the browser to the client's `callback_url` with
   `connected_account_id` and `status`.
4. Actions (`POST /v1/connected-accounts/{id}/actions/{action}`) run with the connection's token.

### Completeness

A connection stores its tokens and the name of its pending refresh task in separate columns.
Whether it needs a task depends on its provider, which declares whether it issues refresh tokens
(`issues_refresh_tokens` in its OAuth definition):

- **A provider that issues refresh tokens** (GitHub, Linear and Slack): the connection is complete
  only when it holds a refresh token and its refresh task. If the consent returns no refresh token
  (for example, the app has expiring tokens or token rotation turned off), or the first task
  cannot be scheduled, osiris keeps no tokens and the connection is `disabled`; the user must
  connect again.
- **A provider that issues none** (its tokens never expire, like Trello's): the connection is
  complete with its access token alone, and nothing is scheduled.

A connection that is not complete is never reported as connected: the API returns
`setup_complete: false` and the status `disabled`. A database check
(`connected_account_complete`) refuses an active connection whose refresh token has no refresh
task.

## Keeping tokens fresh

Each connection with a refresh token has exactly one pending `connection.refresh` task.

- **When it runs**: a margin (`[refresh] margin`) before the access token expires. A token that
  does not expire, or not within 29 days, gets a `keepalive` task after `[refresh] keepalive`
  (29 days, under Cloud Tasks' 30-day limit). That also keeps its refresh token from ageing out.
- **Generation**: every token write increases the connection's `token_generation`. A task names
  the generation it was created for.
- **Checks first**: under a row lock, the worker checks that the connection still exists, is
  active, and is on the task's generation. If not (it was disconnected, reconnected, or already
  refreshed during an action), the task is acknowledged as `skipped_stale` and nothing changes.
- **Success**: the provider returns new tokens. Osiris schedules the next task (generation + 1),
  then saves the new tokens and the next task's name in one write, and only then answers 204. The
  previous task, if it is not the one running, is cancelled.
- **The next task cannot be scheduled**: the new tokens are still saved, because the provider may
  already have invalidated the old refresh token, but the connection becomes `disabled` with no
  task and the worker answers 500. Cloud Tasks retries the same task. The retry finds the
  connection disabled with tokens and no task, schedules the refresh for the new tokens, and makes
  the connection active again (outcome `rescheduled`) without asking the provider again. An
  action that uses the connection does the same. Meanwhile the connection is not usable, and the
  previous task is kept so it can also restore it.
- **The provider fails, depending on its answer**:
  - An outage or a timeout (network errors, 408, 429, 5xx) is temporary. The connection stays
    active, the failure is counted, the old tokens and task stay, and the worker answers 500 so
    Cloud Tasks tries again.
  - A hard refusal (`invalid_grant`, a revoked grant, any other 4xx) means the grant is dead. The
    connection becomes `expired`, its task is cleared, `connection.expired` is published, and
    nothing is rescheduled.
- **During an action**: a token that expires within the margin is refreshed first, in the same
  way. This is only a safety net; the task normally refreshes it in time. A token the provider
  rejects (401) is refreshed once and the action tried again. If it is still rejected, the grant
  no longer exists: the connection becomes `expired`, with `connection.revoked`.
- **Disconnect** (`DELETE /v1/connected-accounts/{id}`): the grant is ended at the provider, the
  connection is deleted, and its pending task is cancelled by name. If cancelling fails, the task
  finds no connection when it runs and is acknowledged as stale.
- **Refresh now** (`POST /v1/connected-accounts/{id}/refresh`, admin key, `osiris connections
  refresh`): schedules an immediate `retry` task that replaces the scheduled one, so there is
  still one pending refresh. The connection's `refresh_task` and `refresh_task_at` show what is
  scheduled.

## Events

Events are stateless: an event says what happened and to which connection, never the
connection's data, and the client fetches the connection for its current state. An event is
published right after the change that caused it. If publishing fails, osiris schedules an
`event.publish` task, named after the event, which publishes it again; the event is lost only if
that task cannot be scheduled either, and this is logged. Clients ignore an event they have already
received, by its `event_id`.

## Idempotency keys

A POST or DELETE with an `Idempotency-Key` runs once, and a retry gets the first answer again. A
key older than `[idempotency] retention` is treated as unused. Expired keys are deleted a few at a
time whenever a new key is written, so no clean-up job is needed.

## Encryption

A master key, derived from `OSIRIS_ENCRYPTION_KEY`, wraps one random data key per tenant: each of
a client's users, and osiris itself for client secrets. Tokens, client secrets and PKCE verifiers
are encrypted with AES-256-GCM and a fresh nonce. Encryption and decryption happen inside
storage, so no code outside it handles encrypted values. In memory they are `Secret`, which never
prints.
