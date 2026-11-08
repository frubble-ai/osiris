#!/usr/bin/env python3
"""Fails when a file outside a provider's own folders names that provider.

The providers are the folders of crates/osiris_providers/src/ that hold a `registration.rs`. A
file belongs to a provider when a segment of its path is the provider's id, or its name is the
id (`<id>.rs`, `<id>.md`). Anywhere else, the only line that may name a provider is a module
declaration (`pub mod <id>;`). Generated artifacts and the files about the platform osiris is
hosted on (whose name a provider may share) are not checked; each is listed below with its reason.

Usage, from the repository's root: python3 scripts/placement-check.py"""
import os, re, subprocess, sys

PROVIDERS_SOURCE = "crates/osiris_providers/src"

# Not checked, each for its reason.
EXEMPT = [
    # Generated from the code, which is checked: the API's specs and golden transcripts, the
    # client, the query cache and the lockfiles.
    "api-reference/",
    "clients/typescript/src/generated/",
    ".sqlx/",
    "Cargo.lock",
    "clients/typescript/package-lock.json",
    # The init migration seeded the first providers' rows and has been applied; migrations never
    # change. Rows are written from the registrations now.
    "migrations/20261104210756_init.up.sql",
    # About the platform osiris is hosted and built on, not a provider: its workflows, owners,
    # secret scan, package registry, dependency sources and job summary.
    ".github/",
    ".gitleaksignore",
    "deny.toml",
    "clients/typescript/publish-release.mjs",
    "clients/typescript/publish-release.test.mjs",
    # This check, which names the hosting platform's references it leaves out, and its tests.
    "scripts/placement-check.py",
    "scripts/test_placement_check.py",
]


# References to the hosting platform, which a provider may share a name with: its `.github/`
# folder, its domains and its CI variables. They are removed from a line before it is checked.
PLATFORM = re.compile(r"\.github/|[\w.-]*github\.(com|io)\b|\bGITHUB_[A-Z_]+")


def providers(root):
    source = os.path.join(root, PROVIDERS_SOURCE)
    return sorted(
        name
        for name in os.listdir(source)
        if os.path.isfile(os.path.join(source, name, "registration.rs"))
    )


def belongs(path, ids):
    parts = path.split("/")
    stem = os.path.splitext(parts[-1])[0]
    return any(part in ids for part in parts[:-1]) or stem in ids


def violations(path, text, ids):
    """The lines of `text` (the file at `path`) that name a provider it doesn't belong to."""
    if belongs(path, ids) or any(path.startswith(exempt) or path == exempt for exempt in EXEMPT):
        return []
    named = re.compile(r"(?i)" + "|".join(re.escape(id) for id in ids))
    declaration = re.compile(r"^\s*(pub(\(crate\))? )?mod (" + "|".join(ids) + r");\s*$")
    found = []
    if named.search(path):
        found.append((0, f"its path names a provider: {path}"))
    for number, line in enumerate(text.splitlines(), start=1):
        if named.search(PLATFORM.sub("", line)) and not declaration.match(line):
            found.append((number, line.strip()))
    return found


def tracked(root):
    listed = subprocess.run(
        ["git", "ls-files", "--cached", "--others", "--exclude-standard"],
        cwd=root,
        check=True,
        capture_output=True,
        text=True,
    )
    return [path for path in listed.stdout.splitlines() if os.path.isfile(os.path.join(root, path))]


def main(root="."):
    ids = providers(root)
    failed = 0
    for path in tracked(root):
        try:
            with open(os.path.join(root, path), encoding="utf-8") as file:
                text = file.read()
        except UnicodeDecodeError:
            text = ""
        for number, line in violations(path, text, ids):
            print(f"{path}:{number}: {line}")
            failed += 1
    if failed:
        print(f"{failed} place(s) name a provider outside its folders; see docs/style-guide/placement-rule.md")
        return 1
    return 0


if __name__ == "__main__":
    sys.exit(main())
