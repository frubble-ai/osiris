#!/usr/bin/env bash
# Drops what the migrations made and runs them again, as one execution of the
# migration job with `--reset`, on the image the job already has. The process
# refuses `--reset` unless OSIRIS_ENV is local, dev or stg. The services then
# roll onto that same image, worker first, so the revision that serves matches
# the schema just made.
set -euo pipefail
: "${MIGRATE_JOB:?}" "${SERVICE:?}" "${WORKER_SERVICE:?}" "${API_HOSTNAME:?}" "${REGION:?}" "${PROJECT:?}"

# The job's image is the one the last deploy built; its tag is that commit.
IMAGE=$(gcloud run jobs describe "$MIGRATE_JOB" --project "$PROJECT" --region "$REGION" \
  --format='value(spec.template.spec.template.spec.containers[0].image)')
[ -n "$IMAGE" ] || { echo "::error::$MIGRATE_JOB has no image"; exit 1; }
case "$IMAGE" in
  *@*) echo "::error::$MIGRATE_JOB's image has no tag to read the commit from: $IMAGE"; exit 1 ;;
esac
export IMAGE COMMIT="${IMAGE##*:}"

# --args replaces the job's arguments for this execution only; its command,
# env, secrets and identity stay the platform repository's. --wait returns
# non-zero when the execution fails.
gcloud run jobs execute "$MIGRATE_JOB" --project "$PROJECT" --region "$REGION" \
  --args=--reset --wait --quiet
echo "✅ $MIGRATE_JOB reset the database and migrated it on $IMAGE"

# The worker first, so it reads what the new API publishes before the API
# publishes it.
scripts=$(dirname "$0")
SERVICE="$WORKER_SERVICE" API_HOSTNAME="" "$scripts/rollout.sh"
"$scripts/rollout.sh"
