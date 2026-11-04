# The everyday commands. `just` alone lists them.
set dotenv-load

default:
    @just --list

# Start the local Postgres and apply the migrations.
up:
    docker compose up -d --wait
    sqlx migrate run

# Stop the local Postgres (its data stays in the volume).
down:
    docker compose down

# Run the service, rebuilding on change.
dev:
    bacon run -- serve

# A new migration file in migrations/.
migration name:
    sqlx migrate add -r {{name}}

# Apply the pending migrations.
migrate:
    sqlx migrate run

# Everything CI checks.
check: fmt-check lint deny test sqlx-check

fmt:
    cargo fmt

fmt-check:
    cargo fmt --check

lint:
    cargo clippy --all-targets -- -D warnings

deny:
    cargo deny check

test:
    cargo nextest run

# Refresh .sqlx/ after changing a query, so CI builds without a database.
sqlx-prepare:
    cargo sqlx prepare -- --all-targets

sqlx-check:
    cargo sqlx prepare --check -- --all-targets
