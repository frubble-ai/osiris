#!/usr/bin/env bash
# Drops every table and enum the database user owns and runs the migrations
# again, as one execution of the migration job, on the image the job already
# has. The execution sets
# OSIRIS_MIGRATE_RESET=true and OSIRIS_MIGRATE_RESET_DATABASE to the job's
# DB_NAME. The job's own environment has neither, so a deploy's migration
# does not reset. The services then roll onto that same image,
# worker first, so the revision that serves matches the schema just made.
set -euo pipefail
: "${MIGRATE_JOB:?}" "${SERVICE:?}" "${WORKER_SERVICE:?}" "${API_HOSTNAME:?}" "${REGION:?}" "${PROJECT:?}"
: "${GITHUB_REPOSITORY:?}" "${GH_TOKEN:?}"

# The job's image is the one the last deploy built; its tag is that commit.
IMAGE=$(gcloud run jobs describe "$MIGRATE_JOB" --project "$PROJECT" --region "$REGION" \
  --format='value(spec.template.spec.template.spec.containers[0].image)')
[ -n "$IMAGE" ] || { echo "::error::$MIGRATE_JOB has no image"; exit 1; }
case "$IMAGE" in
  *@*) echo "::error::$MIGRATE_JOB's image has no tag to read the commit from: $IMAGE"; exit 1 ;;
esac
export IMAGE COMMIT="${IMAGE##*:}"
if [[ ! "$COMMIT" =~ ^[0-9a-f]{40}$ ]]; then
  echo "::error::$MIGRATE_JOB's image tag is not a commit: $IMAGE"
  exit 1
fi

# An image from before OSIRIS_MIGRATE_RESET_DATABASE ignores the variable,
# migrates and exits 0, and nothing is dropped. Refuse that image before
# executing. The checkout has no credentials, so this read uses the workflow's token.
encoded=$(gh api --jq .content \
  "repos/${GITHUB_REPOSITORY}/contents/crates/osiris_server/src/config/migrate.rs?ref=${COMMIT}") \
  || { echo "::error::could not read migrate.rs at $COMMIT (deploy an image that resets, then run this again)"; exit 1; }
decoded=$(printf '%s' "$encoded" | base64 -d)
case "$decoded" in
  *OSIRIS_MIGRATE_RESET_DATABASE*) ;;
  *)
    echo "::error::the migrate job's image $COMMIT does not check OSIRIS_MIGRATE_RESET_DATABASE. Deploy that image first. An older one ignores the variable and exits 0 without dropping tables."
    exit 1
    ;;
esac

# The name the job will connect as. The execution must name that same database,
# or the process refuses the reset.
database=$(gcloud run jobs describe "$MIGRATE_JOB" --project "$PROJECT" --region "$REGION" \
  --format=json | jq -r '.spec.template.spec.template.spec.containers[0].env[]? | select(.name == "DB_NAME") | .value')
if [ -z "$database" ] || [ "$database" = "null" ]; then
  echo "::error::$MIGRATE_JOB has no DB_NAME"
  exit 1
fi

# --update-env-vars applies to this execution only. The job's command, its
# other env, its secrets and its identity stay the platform repository's.
# --wait returns non-zero when the execution fails.
gcloud run jobs execute "$MIGRATE_JOB" --project "$PROJECT" --region "$REGION" \
  --update-env-vars="OSIRIS_MIGRATE_RESET=true,OSIRIS_MIGRATE_RESET_DATABASE=${database}" --wait --quiet
echo "✅ $MIGRATE_JOB reset the database and migrated it on $IMAGE"

# The worker first, so it reads what the new API publishes before the API
# publishes it.
scripts=$(dirname "$0")
SERVICE="$WORKER_SERVICE" API_HOSTNAME="" "$scripts/rollout.sh"
"$scripts/rollout.sh"
