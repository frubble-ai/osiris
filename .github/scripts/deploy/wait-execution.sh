#!/usr/bin/env bash
# Waits until a Cloud Run execution has stopped. `gcloud run jobs execute --wait`
# would keep running after this job is killed, and the concurrency group would
# then let the next deploy or reset start against a migration that is still
# going. On SIGTERM this cancels the execution and keeps waiting for it to stop.
set -euo pipefail
execution="${1:?}"
: "${PROJECT:?}" "${REGION:?}"

cancel_requested=0
on_term() {
  cancel_requested=1
  gcloud run jobs executions cancel "$execution" --project "$PROJECT" --region "$REGION" --quiet || true
}
trap on_term TERM INT

completion_time() {
  gcloud run jobs executions describe "$execution" --project "$PROJECT" --region "$REGION" \
    --format='value(status.completionTime)' || true
}

while [ -z "$(completion_time)" ]; do
  sleep 5
done
trap - TERM INT

if [ "$cancel_requested" -eq 1 ]; then
  echo "::error::$execution was cancelled because this job stopped waiting"
  exit 1
fi

describe=$(gcloud run jobs executions describe "$execution" --project "$PROJECT" --region "$REGION" --format=json)
failed=$(printf '%s' "$describe" | jq -r '.status.failedCount // 0')
cancelled=$(printf '%s' "$describe" | jq -r '.status.cancelledCount // 0')
if [ "$failed" != "0" ] || [ "$cancelled" != "0" ]; then
  echo "::error::$execution did not succeed (failed=$failed cancelled=$cancelled)"
  exit 1
fi
