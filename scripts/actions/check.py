#!/usr/bin/env python3
"""Fails when the committed actions differ from what the generator writes: it runs
generate.py and `cargo fmt`, then compares every generated folder with what was there before.
A hand edit to a generated file, or a spec changed without generating, fails the build.

Usage, from the repository's root: python3 scripts/actions/check.py"""
import hashlib, os, subprocess, sys

GENERATED = [
    "crates/osiris_server/src/use_cases/cases",
    "crates/osiris_server/src/routes/actions",
    "docs/providers",
]


def snapshot():
    files = {}
    for folder in GENERATED:
        for root, _, names in os.walk(folder):
            for name in names:
                path = os.path.join(root, name)
                with open(path, "rb") as file:
                    files[path] = hashlib.sha256(file.read()).hexdigest()
    return files


def main():
    before = snapshot()
    subprocess.run([sys.executable, "scripts/actions/generate.py"], check=True, stdout=subprocess.DEVNULL)
    subprocess.run(["cargo", "fmt"], check=True)
    after = snapshot()
    changed = sorted(path for path in before.keys() | after.keys() if before.get(path) != after.get(path))
    if changed:
        for path in changed:
            print(f"{path}: not what scripts/actions/generate.py writes")
        print(f"{len(changed)} generated file(s) were out of date and are now regenerated; commit them")
        return 1
    return 0


if __name__ == "__main__":
    sys.exit(main())
