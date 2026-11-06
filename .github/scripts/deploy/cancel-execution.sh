#!/usr/bin/env bash
# Runs when the job is cancelled, including by timeout-minutes. The step that
# was waiting may already have been killed, so this repeats the cancel and
# waits until the execution has stopped. GitHub kills this step five minutes
# after the cancellation, so the loop cannot hold the concurrency group past
# that. The migrate process holds a session advisory lock until it exits, and
# a second execution waits on that lock. A failed cancel or a failed status
# read is retried for as long as this step is allowed to run.
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
