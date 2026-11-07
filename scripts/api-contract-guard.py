#!/usr/bin/env python3
"""Holds every stable release to its contract, against the base branch.

For each release whose spec in api-reference/versions/ is stable on the base branch:

- its spec must still be there and still be stable, unless the pull request sunsets it: it is
  labelled `api-sunset` (OSIRIS_SUNSET=true, set by CI) and the release is past the sunset its
  answers advertise, the next release's date plus `support_days` (config.example.toml's default,
  or OSIRIS_SUPPORT_DAYS for a deployment that sets its own; OSIRIS_TODAY sets the day, for tests);
- oasdiff must find no breaking change and no warning between its base and head spec, with the
  events webhook read as an answer osiris sends (oasdiff reads a webhook's payload as a request,
  whose rules point the other way);
- its golden transcripts in api-reference/golden/<release>/ must be unchanged: new ones may be
  added, none changed or removed.

A preview release's changes are listed, never refused. Usage:
  scripts/api-contract-guard.py <base-ref> [oasdiff binary]

A base ref that can't be read exits 2, so a missing fetch never reads as a clean pass.
scripts/test_api_contract_guard.py holds the script to each case.
"""

import datetime
import json
import os
import re
import subprocess
import sys
import tempfile

VERSIONS = "api-reference/versions"
GOLDEN = "api-reference/golden"


def git(*args):
    return subprocess.run(["git", *args], check=True, capture_output=True, text=True).stdout


def base_file(base, path):
    try:
        return json.loads(git("show", f"{base}:{path}"))
    except subprocess.CalledProcessError:
        return None


def stability(spec):
    return (spec or {}).get("info", {}).get("x-osiris-stability", "preview")


def support_days():
    """The support window: OSIRIS_SUPPORT_DAYS, or the default config.example.toml documents."""
    if os.environ.get("OSIRIS_SUPPORT_DAYS"):
        return int(os.environ["OSIRIS_SUPPORT_DAYS"])
    with open("config.example.toml") as file:
        found = re.search(r"^support_days\s*=\s*(\d+)", file.read(), re.M)
    if not found:
        sys.exit("::error::config.example.toml documents no support_days")
    return int(found.group(1))


def released_on(release):
    return datetime.date.fromisoformat(release[:10])


def sunset_problem(release, listed):
    """Why `release` can't be removed yet, or None once it is past its sunset."""
    names = [file.removesuffix(".json") for file in sorted(listed)]
    later = names[names.index(release) + 1:]
    if not later:
        return f"{release} is the newest release, so it has no sunset yet"
    sunset = released_on(later[0]) + datetime.timedelta(days=support_days())
    today = datetime.date.fromisoformat(os.environ["OSIRIS_TODAY"]) if os.environ.get("OSIRIS_TODAY") else datetime.date.today()
    if today < sunset:
        return f"{release}'s answers advertise a sunset of {sunset}, and it is {today}"
    return None


def as_answers(spec):
    """The spec with each webhook also written as a route answering its payload, so that oasdiff
    holds what osiris sends to the rules of what it answers."""
    spec = json.loads(json.dumps(spec))
    for name, item in (spec.get("webhooks") or {}).items():
        for method, operation in item.items():
            schema = operation["requestBody"]["content"]["application/json"]["schema"]
            spec.setdefault("paths", {})[f"/webhooks/{name}"] = {
                "get": {
                    "operationId": f"webhook_{name}_{method}",
                    "responses": {
                        "200": {
                            "description": "What osiris sends",
                            "content": {"application/json": {"schema": schema}},
                        }
                    },
                }
            }
    spec.pop("webhooks", None)
    return spec


def oasdiff(binary, command, base_spec, head_spec, *flags):
    with tempfile.TemporaryDirectory() as directory:
        paths = []
        for name, spec in (("base", base_spec), ("head", head_spec)):
            path = os.path.join(directory, f"{name}.json")
            with open(path, "w") as file:
                json.dump(as_answers(spec), file)
            paths.append(path)
        return subprocess.run([binary, command, *paths, *flags], capture_output=True, text=True)


def summary(text):
    print(text)
    target = os.environ.get("GITHUB_STEP_SUMMARY")
    if target:
        with open(target, "a") as file:
            file.write(text + "\n")


def main():
    base = sys.argv[1] if len(sys.argv) > 1 else "origin/dev"
    binary = sys.argv[2] if len(sys.argv) > 2 else "oasdiff"
    sunset = os.environ.get("OSIRIS_SUNSET") == "true"
    failures = []

    # A base that can't be read must fail, never look like one with nothing to check.
    try:
        git("rev-parse", "--verify", "--quiet", f"{base}^{{commit}}")
    except subprocess.CalledProcessError:
        print(f"::error::the base {base!r} is not a commit this checkout has; fetch it first")
        sys.exit(2)
    # Only a base that has no releases yet has nothing to check.
    has_versions = subprocess.run(["git", "cat-file", "-e", f"{base}:{VERSIONS}"], capture_output=True).returncode == 0
    listed = git("ls-tree", "--name-only", f"{base}:{VERSIONS}/").split() if has_versions else []

    for file in sorted(listed):
        release = file.removesuffix(".json")
        path = f"{VERSIONS}/{file}"
        base_spec = base_file(base, path)
        head_spec = json.load(open(path)) if os.path.exists(path) else None

        if stability(base_spec) != "stable":
            if head_spec is not None:
                changes = oasdiff(binary, "changelog", base_spec, head_spec).stdout.strip()
                summary(f"### {release} (preview): may change in any way\n\n```\n{changes or 'no changes'}\n```\n")
            continue

        if head_spec is None:
            if not sunset:
                failures.append(f"{release} is stable and its spec was removed; a release is only removed past its sunset, in a pull request labelled api-sunset")
            elif problem := sunset_problem(release, listed):
                failures.append(f"{release} can't be removed yet: {problem}")
            continue
        if stability(head_spec) != "stable":
            failures.append(f"{release} was stable and is not any more; a stable release stays stable")

        result = oasdiff(binary, "breaking", base_spec, head_spec, "--fail-on", "WARN")
        if result.returncode != 0:
            failures.append(f"{release} is stable and this would break it; make it a version change of the next release instead (docs/api-versioning.md):\n{result.stdout}{result.stderr}")
        else:
            summary(f"### {release} (stable): nothing breaks\n")

        changed = git("diff", "--name-status", f"{base}...HEAD", "--", f"{GOLDEN}/{release}/").split("\n")
        for line in filter(None, changed):
            status, _, name = line.partition("\t")
            if status != "A":
                failures.append(f"{release} is stable and its golden transcript {name} was changed ({status}); what a stable release does never changes")

    for failure in failures:
        print(f"::error::{failure.splitlines()[0]}")
        print(failure, file=sys.stderr)
    sys.exit(1 if failures else 0)


if __name__ == "__main__":
    main()
