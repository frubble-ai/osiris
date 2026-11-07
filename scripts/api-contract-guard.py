#!/usr/bin/env python3
"""Holds every stable release to its contract, against the base branch.

For each release whose spec in api-reference/versions/ is stable on the base branch:

- its spec must still be there and still be stable, unless the pull request sunsets it: it is
  labelled `api-sunset` (OSIRIS_SUNSET=true, set by CI) and the release is past the sunset its
  answers advertised: the next release's date plus `support_days`. The window is the longest of the
  base's documented default (config.example.toml, what clients were told), the head's, and
  OSIRIS__API__SUPPORT_DAYS (the setting a deployment overrides it with), so a pull request can't
  shorten the promise it is checked against; with none of them known, the removal is refused. The
  sunset a superseded release's spec recorded (`info.x-osiris-sunset`) is kept as the promise:
  a later window shorter than it never brings the removal forward. OSIRIS_TODAY sets the day, for
  tests. A removal deletes the release's transcripts; it never edits one;
- a sunset its spec recorded may only stay or move later;
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
    """Runs git with `args` and answers what it printed."""
    return subprocess.run(["git", *args], check=True, capture_output=True, text=True).stdout


def base_file(base, path):
    """The JSON file at `path` on `base`, or None when the base has none."""
    try:
        return json.loads(git("show", f"{base}:{path}"))
    except subprocess.CalledProcessError:
        return None


def stability(spec):
    """A spec's stability, `preview` when it names none."""
    return (spec or {}).get("info", {}).get("x-osiris-stability", "preview")


def documented_support_days(text):
    """The `support_days` a config.example.toml documents, or None."""
    found = re.search(r"^support_days\s*=\s*(\d+)", text or "", re.M)
    return int(found.group(1)) if found else None


def support_days(base):
    """The longest support window clients may have been promised, or None when none is known."""
    try:
        base_text = git("show", f"{base}:config.example.toml")
    except subprocess.CalledProcessError:
        base_text = None
    head_text = open("config.example.toml").read() if os.path.exists("config.example.toml") else None
    known = [documented_support_days(base_text), documented_support_days(head_text)]
    if os.environ.get("OSIRIS__API__SUPPORT_DAYS"):
        known.append(int(os.environ["OSIRIS__API__SUPPORT_DAYS"]))
    known = [days for days in known if days is not None]
    return max(known) if known else None


def released_on(release):
    """The day a release came out: the date its name starts with."""
    return datetime.date.fromisoformat(release[:10])


def recorded_sunset(spec):
    """The sunset a superseded release's spec recorded, or None."""
    recorded = (spec or {}).get("info", {}).get("x-osiris-sunset")
    return datetime.date.fromisoformat(recorded) if recorded else None


def sunset_problem(base, release, listed, base_spec):
    """Why `release` can't be removed yet, or None once it is past its sunset: the later of the
    sunset its spec recorded and the next release's date plus the longest window known."""
    names = [file.removesuffix(".json") for file in sorted(listed)]
    later = names[names.index(release) + 1:]
    if not later:
        return f"{release} is the newest release, so it has no sunset yet"
    days = support_days(base)
    if days is None:
        return "no support_days is documented or set, so its sunset is unknown"
    candidates = [released_on(later[0]) + datetime.timedelta(days=days), recorded_sunset(base_spec)]
    sunset = max(day for day in candidates if day is not None)
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
    """Runs oasdiff's `command` between the two specs, each with its webhooks read as answers."""
    with tempfile.TemporaryDirectory() as directory:
        paths = []
        for name, spec in (("base", base_spec), ("head", head_spec)):
            path = os.path.join(directory, f"{name}.json")
            with open(path, "w") as file:
                json.dump(as_answers(spec), file)
            paths.append(path)
        return subprocess.run([binary, command, *paths, *flags], capture_output=True, text=True)


def check_transcripts(base, release, failures, removed):
    """Adds a failure for each of `release`'s transcripts changed since `base`: a new one may be
    added, and a removed release's are deleted, but none is ever edited."""
    allowed = {"A", "D"} if removed else {"A"}
    changed = git("diff", "--name-status", f"{base}...HEAD", "--", f"{GOLDEN}/{release}/").split("\n")
    for line in filter(None, changed):
        status, _, name = line.partition("\t")
        if status not in allowed:
            failures.append(f"{release} is stable and its golden transcript {name} was changed ({status}); what a stable release does never changes")


def summary(text):
    """Prints `text`, and adds it to the job's summary on GitHub."""
    print(text)
    target = os.environ.get("GITHUB_STEP_SUMMARY")
    if target:
        with open(target, "a") as file:
            file.write(text + "\n")


def main():
    """Checks every release stable on the base, and exits 1 on any failure."""
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
            elif problem := sunset_problem(base, release, listed, base_spec):
                failures.append(f"{release} can't be removed yet: {problem}")
            check_transcripts(base, release, failures, removed=True)
            continue
        if stability(head_spec) != "stable":
            failures.append(f"{release} was stable and is not any more; a stable release stays stable")
        promised, now = recorded_sunset(base_spec), recorded_sunset(head_spec)
        if promised and (now is None or now < promised):
            failures.append(f"{release}'s spec recorded a sunset of {promised}, and this moves it to {now}; a promised sunset is never brought forward")

        result = oasdiff(binary, "breaking", base_spec, head_spec, "--fail-on", "WARN")
        if result.returncode != 0:
            failures.append(f"{release} is stable and this would break it; make it a version change of the next release instead (docs/api-versioning.md):\n{result.stdout}{result.stderr}")
        else:
            summary(f"### {release} (stable): nothing breaks\n")

        check_transcripts(base, release, failures, removed=False)

    for failure in failures:
        print(f"::error::{failure.splitlines()[0]}")
        print(failure, file=sys.stderr)
    sys.exit(1 if failures else 0)


if __name__ == "__main__":
    main()
