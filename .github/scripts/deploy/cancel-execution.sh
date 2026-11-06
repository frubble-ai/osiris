#!/usr/bin/env bash
# Runs when the job is cancelled, including by timeout-minutes. The step that
# was waiting may already have been killed, so this repeats the cancel and
# does not return until the execution has stopped. The concurrency group stays
# held until then. A failed cancel or a failed status read is retried: leaving
# this step while the execution might still be running would let the next
# deploy or reset start against it.
set -euo pipefail
: "${PROJECT:?}" "${REGION:?}" "${RUNNER_TEMP:?}"

file="${RUNNER_TEMP}/osiris-job-execution"
[ -f "$file" ] || exit 0
execution=$(tr -d '[:space:]' < "$file")
[ -n "$execution" ] || exit 0

while true; do
  if completion=$(gcloud run jobs executions describe "$execution" --project "$PROJECT" --region "$REGION" \
    --format='value(status.completionTime)'); then
    if [ -n "$completion" ]; then
      exit 0
    fi
  else
    echo "::warning::could not read execution $execution"
  fi
  if ! gcloud run jobs executions cancel "$execution" --project "$PROJECT" --region "$REGION" --quiet; then
    echo "::warning::could not cancel execution $execution"
  fi
  sleep 5
done
