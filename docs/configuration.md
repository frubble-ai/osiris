# Configuration

Osiris reads its configuration from environment variables. The image contains no settings files
and no secrets, and what differs between environments is the values of the variables.

## Choosing the process

One image runs every process. `OSIRIS_PROCESS` is required and chooses which.

| `OSIRIS_PROCESS` | Runs as | Does |
| --- | --- | --- |
| `serve` | Cloud Run service, public | the API and the OAuth callback, on `PORT` |
| `worker` | Cloud Run service, internal | `POST /tasks` for the task scheduler, on `PORT` |
| `migrate` | Cloud Run job | applies the pending migrations, then exits 0 (non-zero on failure). `OSIRIS_MIGRATE_RESET=true` drops this user's tables and enums in `public` first |

A missing or unknown value stops the process with an error naming `OSIRIS_PROCESS`. `serve` and
`worker` listen on `0.0.0.0:$PORT`. After SIGTERM they stop accepting connections, let requests
already started finish for at most `[server] shutdown_grace_seconds` (8), then exit.

## Environment variables

| Variable | Needed by | What |
| --- | --- | --- |
| `OSIRIS_PROCESS` | all | `serve`, `worker` or `migrate` |
| `OSIRIS_MIGRATE_RESET` | migrate, optional | `true` exactly drops the tables and enums this database user owns in `public`, then migrates. Any other value, or unset, only migrates. The reset workflow sets it for one execution; the job itself never has it, and a deploy does not set it |
| `OSIRIS_MIGRATE_RESET_DATABASE` | migrate, when resetting | the database that reset must be connected to. Required with `OSIRIS_MIGRATE_RESET=true`, and it must equal the database the process connects to (`DB_NAME`, and the session's `current_database()`). A missing or different name refuses the reset before anything is dropped |
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
  services. The process takes a session advisory lock and runs the drop and the migrations on
  that same connection, which it keeps open until it exits. A second execution waits for the
  lock before it drops or migrates. If PostgreSQL closes the session, the lock and the work
  stop together. GitHub kills a cancelled job's cleanup after five minutes, which releases the
  workflow's concurrency group; the lock is what still keeps the next execution off the schema
  while the earlier one is running.
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

## Continuous deployment

[`.github/workflows/osiris-cd.yml`](../.github/workflows/osiris-cd.yml) builds that image and rolls
dev or stg onto it. The file name is part of the deploy trust in the platform repository
(`osiris-cd.yml` in `osiris-dev` from any branch, and in `osiris-stg` from `stg` only). Renaming
the file stops the Google sign-in.

| Trigger | Environment | Deploys |
| --- | --- | --- |
| Push to `dev` that changes the image, the migrations or this workflow | `osiris-dev` | that commit to dev |
| A run by hand from any branch except `stg` and `prd` | `osiris-dev` | that branch to dev |
| Push to `stg`, or a run by hand from `stg` | `osiris-stg` | that commit to stg |
| A tag, or a run from `prd` | refused | |

The repository variable `OSIRIS_CD_ENABLED` must be `true`, or every job is skipped. One rollout
runs at a time per environment; further runs wait. While the repository variable `STG_ENABLED` is
exactly `false` (kept in step with the same variable on frubble-ai/platform), a run on `stg` is
skipped.

The deploy job signs in with Workload Identity as `GCP_DEPLOY_HUB`, then impersonates
`GCP_DEPLOY_SPOKE`. It pushes `frubble-osiris:<commit>` to `GCP_REGISTRY`, executes
`GCP_RUN_JOB_MIGRATE` on that image, then rolls `GCP_RUN_SERVICE` (serve) and
`GCP_RUN_SERVICE_WORKER`. Each service and the job already have `OSIRIS_PROCESS`, so the deploy
changes the image only. Serve is checked at `https://osiris.<env>.frubble.ai/health`, which
answers the commit. The worker is internal; its revision must be ready and hold the traffic.

`GCP_WIF_PROVIDER` and `GCP_WIF_AUDIENCE` are read from `vars` (the environment, or the repository
when the environment leaves them unset). The other `GCP_*` names above are variables on
`osiris-dev` and `osiris-stg`, the same names `backend-dev` uses, with osiris's own values from
the platform outputs. Authentication is Workload Identity. The database password and the keys stay
in Secret Manager, mounted by Terraform.

The image already runs on Cloud Run. The Dockerfile's entrypoint is the binary, Cloud Run's
`PORT` is 8080, and `/health` is the startup and liveness probe. The deploy sets nothing else
about how the process starts.

## Resetting the database

[`.github/workflows/db-reset.yml`](../.github/workflows/db-reset.yml) drops every table and enum the
database user owns in `public`, not only those the migrations made, and runs the migrations again from nothing,
as one execution of the migration job with `OSIRIS_MIGRATE_RESET=true` and
`OSIRIS_MIGRATE_RESET_DATABASE` set to that database
([`reset-db.sh`](../.github/scripts/deploy/reset-db.sh)), on the image the job already has. It then
rolls the worker, then serve, onto that image. It is for when the migrations' history is rewritten.
Every connection goes with the tables. A reset whose named database is missing or is not the one
the process connects to is refused before anything is dropped.

The job must already run an image whose migrate process checks `OSIRIS_MIGRATE_RESET_DATABASE`.
The script reads that commit's `config/migrate.rs` and refuses to execute when the variable is not
there: an older image ignores it, migrates and exits 0, and no table is dropped. Deploy first (its
migration moves the job onto the new image), then reset.

The process does not read which environment it is running in. prd is protected because this
workflow runs only from `dev` and `stg`, and the platform's trust names `db-reset.yml` only for
`osiris-dev` and `osiris-stg`. The variable is an override of that one execution, so the next
migration does not reset. A principal who can execute the migrate job with an environment override
can reset that job's database; a prd job must not grant that.

It runs by hand only, from `dev` or `stg`, and only for `drn1996`. It shares that environment's
deploy concurrency group, so it never runs beside a deploy, and it signs in as the deploy
identity. The trust rule in the platform repository names `db-reset.yml`: `osiris-dev` from `dev`
only, `osiris-stg` from `stg` only. Renaming the file stops the Google sign-in. A run on `stg`
while `STG_ENABLED` is exactly `false` is refused.
