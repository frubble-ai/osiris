#!/usr/bin/env bash
# Runs when the job is cancelled, including by timeout-minutes. The step that
# was waiting may already have been killed, so this repeats the cancel and
# does not return until the execution has stopped. The concurrency group stays
# held until then.
set -euo pipefail
: "${PROJECT:?}" "${REGION:?}" "${RUNNER_TEMP:?}"

file="${RUNNER_TEMP}/osiris-job-execution"
[ -f "$file" ] || exit 0
execution=$(tr -d '[:space:]' < "$file")
[ -n "$execution" ] || exit 0

gcloud run jobs executions cancel "$execution" --project "$PROJECT" --region "$REGION" --quiet || true

failures=0
for _ in $(seq 1 60); do
  completion=$(gcloud run jobs executions describe "$execution" --project "$PROJECT" --region "$REGION" \
    --format='value(status.completionTime)') && status=0 || status=$?
  if [ "$status" -ne 0 ]; then
    failures=$((failures + 1))
    if [ "$failures" -ge 12 ]; then
      echo "::error::could not read execution $execution ($failures describe attempts failed)"
      exit 1
    fi
    sleep 5
    continue
  fi
  failures=0
  if [ -n "$completion" ]; then
    exit 0
  fi
  sleep 5
done
echo "::error::$execution was still running after it was cancelled"
exit 1
