#!/usr/bin/env bash
# Runs Schemathesis against a live osiris, once per release it serves and once per key: every
# operation called with generated requests at that release (`Osiris-Version`), each answer checked
# against that release's spec in api-reference/versions/ (documented status, content type, schema,
# headers) and never a server error. Expects osiris's environment (.env.example's values) and a
# built target/debug/osiris-server; starts the server on its own database and stops it after.
#
# Schemathesis and every package it needs are installed into target/schemathesis-venv from
# requirements.txt, each pinned by its hash (`uv pip compile --universal --python-version 3.11
# --generate-hashes requirements.in -o requirements.txt` updates them). uv is used when present,
# for a Python new enough; otherwise python3 (3.11 or later) and pip.
set -euo pipefail

here="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
venv="target/schemathesis-venv"
if [ ! -x "$venv/bin/schemathesis" ] || ! cmp -s "$here/requirements.txt" "$venv/requirements.txt"; then
  rm -rf "$venv"
  if command -v uv >/dev/null; then
    uv venv --quiet --python 3.11 "$venv"
    uv pip install --quiet --python "$venv/bin/python" --require-hashes -r "$here/requirements.txt"
  else
    python3 -m venv "$venv"
    "$venv/bin/pip" install --quiet --require-hashes -r "$here/requirements.txt"
  fi
  cp "$here/requirements.txt" "$venv/requirements.txt"
fi

port="${SCHEMATHESIS_PORT:-18090}"
url="postgres://$DB_USER:$DB_PASSWORD@$DB_HOST:$DB_PORT/${DB_NAME}_schemathesis?sslmode=verify-ca&sslrootcert=$PWD/.local/tls/ca.pem"
export DB_NAME="${DB_NAME}_schemathesis" OSIRIS_PUBLIC_URL="http://localhost:$port"
sqlx database drop -y --database-url "$url" >/dev/null
sqlx database create --database-url "$url"
OSIRIS_PROCESS=migrate target/debug/osiris-server
log="target/schemathesis-server.log"
# Generated requests carry both keys, so they must only reach the server started here: refuse a
# port that already answers.
if curl -s -o /dev/null "http://localhost:$port/health"; then
  echo "something already answers on port $port; set SCHEMATHESIS_PORT to a free one" >&2
  exit 1
fi
OSIRIS_PROCESS=serve PORT="$port" target/debug/osiris-server > "$log" 2>&1 &
server=$!
# Cleanup on the way out: stop the server and wait for it, so its connections are closed before
# its database is dropped. A drop that fails is reported, and the next run starts by dropping it.
# shellcheck disable=SC2329 # called by the EXIT trap below
cleanup() {
  kill "$server" 2>/dev/null || true # the server may have stopped already
  wait "$server" 2>/dev/null || true # its exit status is not this script's
  if ! sqlx database drop -y --database-url "$url" >/dev/null; then
    echo "could not drop ${DB_NAME}; the next run drops it first" >&2
  fi
}
trap cleanup EXIT

healthy=false
for _ in $(seq 1 50); do
  if curl -sf "http://localhost:$port/health" >/dev/null; then
    healthy=true
    break
  fi
  sleep 0.2
done
# The server started here must still be running: one that failed to bind has exited.
if [ "$healthy" != true ] || ! kill -0 "$server" 2>/dev/null; then
  echo "the osiris started here did not answer /health on port $port within 10 seconds; its log:" >&2
  cat "$log" >&2
  exit 1
fi

status=0
for spec in api-reference/versions/*.json; do
  release="$(basename "$spec" .json)"
  for key in "$OSIRIS_API_KEY" "$OSIRIS_ADMIN_KEY"; do
    "$venv/bin/schemathesis" run "$spec" \
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
