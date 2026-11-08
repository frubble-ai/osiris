# The everyday commands. `just` alone lists them.
set dotenv-load

# Osiris reads the database's CA certificate from DB_CA_PEM, in every environment. A value already
# set (in the shell or in .env) is used as it is. Otherwise it is the CA docker-compose generates in
# .local/tls (`just up`). A missing file is not an error here, so that `just --list` works before
# `just up`; osiris itself refuses an empty DB_CA_PEM at startup.
export DB_CA_PEM := env("DB_CA_PEM", `cat .local/tls/ca.pem 2>/dev/null || true`)

# sqlx's tools (sqlx-cli, `cargo sqlx prepare`) and its test harness (`#[sqlx::test]`) only take a
# URL, so one is built here from the same DB_* values, with the same TLS. Osiris itself never reads
# DATABASE_URL.
export DATABASE_URL := "postgres://" + env("DB_USER", "") + ":" + env("DB_PASSWORD", "") + "@" + env("DB_HOST", "") + ":" + env("DB_PORT", "") + "/" + env("DB_NAME", "") + "?sslmode=verify-ca&sslrootcert=" + justfile_directory() + "/.local/tls/ca.pem"

default:
    @just --list

# Start Postgres over TLS (generating its local CA once) and the Pub/Sub emulator with the events
# topic, then apply the migrations with osiris's own migrate process.
up:
    docker compose up -d --wait postgres pubsub
    docker compose run --rm pubsub-setup
    just migrate

# Stop the local stack (Postgres keeps its data in the volume; the emulator starts empty).
down:
    docker compose down

# Run the api (API_PORT, 8090) and the worker (WORKER_PORT, 8091) together, each chosen by
# OSIRIS_PROCESS and listening on PORT as on Cloud Run; Ctrl-C stops both.
dev:
    #!/usr/bin/env bash
    set -euo pipefail
    cargo build -p osiris_server
    trap 'kill 0' EXIT
    OSIRIS_PROCESS=serve PORT="${API_PORT:-8090}" target/debug/osiris-server &
    OSIRIS_PROCESS=worker PORT="${WORKER_PORT:-8091}" target/debug/osiris-server &
    wait

# A new migration file in migrations/.
migration name:
    sqlx migrate add -r {{name}}

# Apply the pending migrations, the way the deployed migrate job does.
migrate:
    cargo build -q -p osiris_server
    OSIRIS_PROCESS=migrate target/debug/osiris-server

# Every migration applied, reverted and applied again, on a throwaway database of its own (never
# the development database).
migrations-check:
    #!/usr/bin/env bash
    set -euo pipefail
    url="$(printf '%s' "$DATABASE_URL" | sed "s#/${DB_NAME}?#/${DB_NAME}_migrations_check?#")"
    sqlx database drop -y --database-url "$url"
    sqlx database create --database-url "$url"
    sqlx migrate run --database-url "$url"
    sqlx migrate revert --target-version 0 --database-url "$url"
    sqlx migrate run --database-url "$url"
    sqlx database drop -y --database-url "$url"

# Refresh .sqlx/ after changing a query, so CI and the image build without a database.
sqlx-prepare:
    SQLX_OFFLINE=false cargo sqlx prepare --workspace -- --all-targets

sqlx-check:
    SQLX_OFFLINE=false cargo sqlx prepare --workspace --check -- --all-targets

# Regenerate api-reference/openapi.json (the current release) and api-reference/versions/ (every
# release osiris serves) from the routes.
openapi:
    cargo run -q -p osiris_openapi

# Every stable release against `base` (default origin/dev): its spec may only grow, and its golden
# transcripts never change. Needs oasdiff on the PATH.
contract-check base="origin/dev":
    python3 -m unittest scripts/test_api_contract_guard.py
    scripts/api-contract-guard.py {{base}} oasdiff

# Schemathesis against a live osiris at every release and key, on a database of its own.
schemathesis:
    cargo build -q -p osiris_server
    scripts/schemathesis/run.sh

# Everything CI checks.
check: fmt-check lint deny typos sqlx-check test openapi-check

fmt:
    cargo fmt

fmt-check:
    cargo fmt --check

lint:
    cargo clippy --workspace --all-targets -- -D warnings

deny:
    cargo deny check

typos:
    typos

# The tests; arguments filter them, such as `just test golden`.
test *filters:
    cargo nextest run --workspace {{filters}}

# The committed specs are the ones the routes describe.
openapi-check:
    rm -rf target/api-reference
    cargo run -q -p osiris_openapi target/api-reference
    diff -ru api-reference/openapi.json target/api-reference/openapi.json
    diff -ru api-reference/versions target/api-reference/versions
