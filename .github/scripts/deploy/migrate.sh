#!/usr/bin/env bash
# Runs the database migration as an execution of the migration job, on IMAGE,
# before the services roll onto it. The job's definition (env, secrets, the
# identity it runs as) is the platform repository's; this only moves its image
# and runs it once. OSIRIS_PROCESS=migrate is already set on the job.
#
# A failed migration stops the deploy here: the services keep serving on the
# previous image. sqlx runs each migration in a transaction, so a failed one
# leaves the schema as it found it.
set -euo pipefail
: "${IMAGE:?}" "${REGION:?}" "${PROJECT:?}" "${MIGRATE_JOB:?}"

gcloud run jobs update "$MIGRATE_JOB" --project "$PROJECT" --region "$REGION" \
  --image "$IMAGE" --quiet

# --wait returns non-zero when the execution fails, and prints its name.
gcloud run jobs execute "$MIGRATE_JOB" --project "$PROJECT" --region "$REGION" \
  --wait --quiet
echo "✅ $MIGRATE_JOB ran on $IMAGE"
