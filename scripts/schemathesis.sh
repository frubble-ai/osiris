#!/usr/bin/env bash
# Runs Schemathesis against a live osiris, once per release it serves and once per key: every
# operation called with generated requests at that release (`Osiris-Version`), each answer checked
# against that release's spec in api-reference/versions/ (documented status, content type, schema,
# headers) and never a server error. Expects osiris's environment (.env.example's values) and a
# built target/debug/osiris-server; starts the server on its own database and stops it after.
# SCHEMATHESIS is the command to run it with (by default uvx, at the pinned version).
set -euo pipefail

port="${SCHEMATHESIS_PORT:-18090}"
version="${SCHEMATHESIS_VERSION:-4.33.1}"
url="postgres://$DB_USER:$DB_PASSWORD@$DB_HOST:$DB_PORT/${DB_NAME}_schemathesis?sslmode=verify-ca&sslrootcert=$PWD/.local/tls/ca.pem"
export DB_NAME="${DB_NAME}_schemathesis" OSIRIS_PUBLIC_URL="http://localhost:$port"
sqlx database drop -y --database-url "$url" >/dev/null
sqlx database create --database-url "$url"
OSIRIS_PROCESS=migrate target/debug/osiris-server
OSIRIS_PROCESS=serve PORT="$port" target/debug/osiris-server > target/schemathesis-server.log 2>&1 &
server=$!
trap 'kill $server 2>/dev/null || true; sqlx database drop -y --database-url "$url" >/dev/null || true' EXIT
for _ in $(seq 1 50); do
  curl -sf "http://localhost:$port/health" >/dev/null && break
  sleep 0.2
done

status=0
for spec in api-reference/versions/*.json; do
  release="$(basename "$spec" .json)"
  for key in "$OSIRIS_API_KEY" "$OSIRIS_ADMIN_KEY"; do
    # shellcheck disable=SC2086 # SCHEMATHESIS may be a command with arguments.
    ${SCHEMATHESIS:-uvx --quiet schemathesis==$version} run "$spec" \
      --url "http://localhost:$port" \
      --header "Authorization: Bearer $key" \
      --header "Osiris-Version: $release" \
      --checks not_a_server_error,status_code_conformance,content_type_conformance,response_schema_conformance,response_headers_conformance \
      --max-examples "${SCHEMATHESIS_EXAMPLES:-25}" \
      --suppress-health-check all \
      --no-color || status=1
  done
done
exit "$status"
