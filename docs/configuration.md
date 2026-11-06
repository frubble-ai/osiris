# Configuration

Osiris reads its configuration from environment variables. The image contains no settings files
and no secrets, and what differs between environments is the values of the variables.

## Choosing the process

One image runs every process. `OSIRIS_PROCESS` is required and chooses which.

| `OSIRIS_PROCESS` | Runs as | Does |
| --- | --- | --- |
| `serve` | Cloud Run service, public | the API and the OAuth callback, on `PORT` |
| `worker` | Cloud Run service, internal | `POST /tasks` for the task scheduler, on `PORT` |
| `migrate` | Cloud Run job | applies the pending migrations, then exits 0 (non-zero on failure) |

A missing or unknown value stops the process with an error naming `OSIRIS_PROCESS`. `serve` and
`worker` listen on `0.0.0.0:$PORT`. After SIGTERM they stop accepting connections, let requests
already started finish for at most `[server] shutdown_grace_seconds` (8), then exit.

## Environment variables

| Variable | Needed by | What |
| --- | --- | --- |
| `OSIRIS_PROCESS` | all | `serve`, `worker` or `migrate` |
| `DB_HOST`, `DB_PORT`, `DB_NAME`, `DB_USER` | all | where the database is, and who connects |
| `DB_PASSWORD` | all | the database user's password (a secret) |
| `DB_CA_PEM` | all | the database server's CA certificate, in PEM (see below) |
| `PORT` | serve, worker | the port to listen on; Cloud Run sets it |
| `OSIRIS_ENCRYPTION_KEY` | serve, worker | the master key's secret, 32+ characters |
| `OSIRIS_PUBLIC_URL` | serve, worker | osiris's public origin, used in the OAuth redirect URI |
| `OSIRIS_API_KEY` | serve | the key clients send (32+ characters) |
| `OSIRIS_ADMIN_KEY` | serve | the key operators send (32+ characters); it must differ from `OSIRIS_API_KEY` |
| `PUBSUB_PROJECT`, `OSIRIS_EVENTS_TOPIC` | serve, worker | where events for clients are published |
| `PUBSUB_EMULATOR_HOST` | locally | the Pub/Sub emulator; unset when deployed |
| `OSIRIS_TASKS` | serve, worker | which task scheduler runs scheduled tasks: `cloud_tasks` when deployed, `local` in development |
| `OSIRIS_WORKER_URL` | serve, worker | the worker's origin; scheduled tasks are sent to its `/tasks` |
| `CLOUD_TASKS_PROJECT`, `CLOUD_TASKS_LOCATION`, `CLOUD_TASKS_QUEUE` | serve, worker, with `OSIRIS_TASKS=cloud_tasks` | the Cloud Tasks queue; its retry policy decides when a failed task runs again |
| `CLOUD_TASKS_SERVICE_ACCOUNT` | serve, worker, with `OSIRIS_TASKS=cloud_tasks` | the service account each task's OIDC token is issued for; it must be allowed to invoke the worker |
| `CLOUD_TASKS_AUDIENCE` | serve, worker, with `OSIRIS_TASKS=cloud_tasks`, optional | the OIDC token's audience; `OSIRIS_WORKER_URL` by default |
| `OSIRIS_WORKER_TOKEN` | serve and worker, with `OSIRIS_TASKS=local` | the bearer token (32+ characters) the local scheduler sends and the worker's `/tasks` requires |
| `OSIRIS_COMMIT` | all | the commit the image was built from; the Dockerfile sets it from its `OSIRIS_COMMIT` build argument, and `/health` reports it. `local` when unset |
| `LOG_FORMAT` | all | `json` for one JSON object per line on stdout, with the `severity` field Cloud Logging reads; anything else for readable lines |
| `RUST_LOG` | all | the log filter, `info` when unset |
| `OSIRIS__<SECTION>__<KEY>` | all | changes one setting (see Settings below) |
| `OSIRIS_EVENTS_PUSH_ENDPOINT` | locally (docker-compose's `pubsub-setup`) | where the emulator pushes events; empty means nowhere |
| `OSIRIS_LIVE_GITHUB_TOKEN`, `OSIRIS_LIVE_LINEAR_TOKEN`, `OSIRIS_LIVE_SLACK_TOKEN` | the live provider tests (`live.yml`) | a real access token per provider; a test without its token is skipped locally and fails in CI |

A required variable that is missing or malformed stops the process at startup, naming it.

## The database

The connection is built from `DB_HOST`, `DB_PORT`, `DB_NAME`, `DB_USER`, `DB_PASSWORD` and
`DB_CA_PEM`.

TLS is required in every environment: `sslmode=verify-ca`, trusting `DB_CA_PEM`. If a `DB_*` value
is missing, or the connection or its TLS can't be established within
`[database] connect_timeout_seconds`, the process stops at startup.

Each process opens at most `[database] max_connections` connections (5), and a refresh waits at
most `[database] lock_timeout_seconds` for another refresh's row lock.

## Health

`GET /health` on both services runs a cheap query (`SELECT 1`). It answers 200 with
`{"status": "ok", "database": true, "commit": "…"}` when the database answers, and 503 with
`"database": false` when it doesn't. The worker also answers 503 (`"status": "draining"`) once
SIGTERM has arrived. The `commit` lets a rollout check which revision is answering.

## Settings

Timings and limits have defaults in code, in each section of `config/settings/`, and are validated
at startup. `OSIRIS__<SECTION>__<KEY>` changes one, for example
`OSIRIS__REFRESH__MARGIN_SECONDS=600`. No file is read.

[`config.example.toml`](../config.example.toml) lists every setting with its default and what it
does; a test fails if it and the defaults disagree.

## Running locally

Locally, osiris reads the same variables, against a Postgres that only accepts TLS.

```sh
cp .env.example .env   # the DB_* values, the keys and ports for local development
just up                # generates a local CA in .local/tls, starts Postgres over TLS and the
                       # Pub/Sub emulator, and runs the migrate process
just dev               # OSIRIS_PROCESS=serve on PORT=8090 and OSIRIS_PROCESS=worker on PORT=8091
```

docker-compose's `postgres-tls` generates the CA and a server certificate in `.local/tls`
(git-ignored); Postgres refuses plaintext connections. The justfile exports `DB_CA_PEM` from
`.local/tls/ca.pem`, and `just` loads `.env`. The server reads only the environment. Outside
`just`:

```sh
set -a; . ./.env; set +a
export DB_CA_PEM="$(cat .local/tls/ca.pem)"
OSIRIS_PROCESS=serve PORT=8090 cargo run -p osiris_server
```

sqlx's tools (`sqlx migrate`, `cargo sqlx prepare`) and test harness (`#[sqlx::test]`) take a URL;
the justfile and CI build `DATABASE_URL` from the `DB_*` values, with `sslmode=verify-ca` and the
same CA. Osiris does not read it.

## Deploying

- `serve`: a public Cloud Run service, with `OSIRIS_PROCESS=serve`.
- `worker`: an internal Cloud Run service that requires authentication, with
  `OSIRIS_PROCESS=worker`. `CLOUD_TASKS_SERVICE_ACCOUNT` has `roles/run.invoker` on it. The worker
  also checks each task's OIDC token (Google's signature, the issuer, `CLOUD_TASKS_AUDIENCE`,
  `CLOUD_TASKS_SERVICE_ACCOUNT`).
- `migrate`: a Cloud Run job with `OSIRIS_PROCESS=migrate`, run once per deploy before the
  services.
- The database: the `DB_*` variables, `DB_PASSWORD` from Secret Manager and `DB_CA_PEM` the Cloud
  SQL instance's server CA certificate.
- One Cloud Tasks queue (`CLOUD_TASKS_QUEUE`). The service accounts of `serve` and `worker` have
  `roles/cloudtasks.enqueuer` and `roles/cloudtasks.taskDeleter` on it, and
  `roles/iam.serviceAccountUser` on `CLOUD_TASKS_SERVICE_ACCOUNT`. The queue's retry policy
  (attempts, backoff) retries failed tasks.
- One Pub/Sub topic for events (`OSIRIS_EVENTS_TOPIC`); `serve` and `worker` have
  `roles/pubsub.publisher` on it.
- The image is built with `--build-arg OSIRIS_COMMIT=<commit>`, which `/health` reports, and runs
  with `LOG_FORMAT=json`.
