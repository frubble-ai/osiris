#!/usr/bin/env python3
"""The contract guard, held to each case it decides: each test builds a base commit and a head
commit in a throwaway repository and runs scripts/api-contract-guard.py between them.

Needs oasdiff: on the PATH, or named by OASDIFF. Run with
  python3 -m unittest scripts/test_api_contract_guard.py
"""

import copy
import json
import os
import subprocess
import tempfile
import unittest

GUARD = os.path.join(os.path.dirname(os.path.abspath(__file__)), "api-contract-guard.py")
OASDIFF = os.environ.get("OASDIFF", "oasdiff")
RELEASE = "2026-11-04.anubis"
NEXT = "2027-03-01.bastet"
SPEC = f"api-reference/versions/{RELEASE}.json"
GOLDEN = f"api-reference/golden/{RELEASE}/refusals.json"


def spec(stability, release=RELEASE):
    """A release's spec, as small as a real one's shape allows: one answer and the events webhook."""
    thing = {
        "type": "object",
        "required": ["id", "name"],
        "properties": {"id": {"type": "string"}, "name": {"type": "string"}},
    }
    event = {
        "type": "object",
        "required": ["type", "connected_account_id"],
        "properties": {"type": {"type": "string"}, "connected_account_id": {"type": "string"}},
    }
    return {
        "openapi": "3.1.0",
        "info": {"title": "Osiris", "version": release, "x-osiris-stability": stability},
        "paths": {
            "/v1/things/{id}": {
                "get": {
                    "operationId": "read_thing",
                    "parameters": [{"name": "id", "in": "path", "required": True, "schema": {"type": "string"}}],
                    "responses": {
                        "200": {
                            "description": "The thing",
                            "content": {"application/json": {"schema": {"$ref": "#/components/schemas/Thing"}}},
                        }
                    },
                }
            }
        },
        "webhooks": {
            "events": {
                "post": {
                    "operationId": "receive_event",
                    "requestBody": {
                        "required": True,
                        "content": {"application/json": {"schema": {"$ref": "#/components/schemas/EventDelivery"}}},
                    },
                    "responses": {"200": {"description": "Received"}},
                }
            }
        },
        "components": {"schemas": {"Thing": thing, "EventDelivery": event}},
    }


class Repository:
    """A throwaway repository with a base commit holding `base_files`."""

    def __init__(self, directory, base_files, config=True):
        self.directory = directory
        self.git("init", "-q", "-b", "main")
        documented = {"config.example.toml": "[api]\nsupport_days = 730\n"} if config else {}
        self.write({**documented, **base_files})
        self.commit("base")
        self.base = self.git("rev-parse", "HEAD").strip()

    def git(self, *args):
        return subprocess.run(
            ["git", "-c", "user.name=test", "-c", "user.email=test@example.com", *args],
            cwd=self.directory,
            check=True,
            capture_output=True,
            text=True,
        ).stdout

    def write(self, files):
        for path, content in files.items():
            full = os.path.join(self.directory, path)
            if content is None:
                os.remove(full)
                continue
            os.makedirs(os.path.dirname(full), exist_ok=True)
            with open(full, "w") as file:
                file.write(content if isinstance(content, str) else json.dumps(content, indent=2))

    def commit(self, message):
        self.git("add", "-A")
        self.git("commit", "-q", "--allow-empty", "-m", message)

    def guard(self, base=None, sunset=False, today="2026-11-07", extra=None):
        environment = {name: value for name, value in os.environ.items() if name != "OSIRIS__API__SUPPORT_DAYS"}
        environment.update(OSIRIS_SUNSET="true" if sunset else "false", OSIRIS_TODAY=today, **(extra or {}))
        environment.pop("GITHUB_STEP_SUMMARY", None)
        return subprocess.run(
            ["python3", GUARD, base or self.base, OASDIFF],
            cwd=self.directory,
            capture_output=True,
            text=True,
            env=environment,
        )


class ContractGuardTest(unittest.TestCase):
    def repository(self, stability="stable"):
        directory = tempfile.TemporaryDirectory()
        self.addCleanup(directory.cleanup)
        return Repository(directory.name, {SPEC: spec(stability), GOLDEN: '{"steps": []}\n'})

    def head(self, repository, change=None, files=None):
        changed = spec("stable") if change is None else change(copy.deepcopy(spec("stable")))
        repository.write({SPEC: changed, **(files or {})})
        repository.commit("head")
        return repository.guard()

    def assert_refused(self, result, words):
        self.assertNotEqual(result.returncode, 0, result.stdout + result.stderr)
        self.assertIn(words, result.stdout + result.stderr)

    def assert_passed(self, result):
        self.assertEqual(result.returncode, 0, result.stdout + result.stderr)

    # What a stable release refuses.

    def test_refuses_an_answer_field_removed_from_a_stable_release(self):
        def remove_name(changed):
            thing = changed["components"]["schemas"]["Thing"]
            del thing["properties"]["name"]
            thing["required"].remove("name")
            return changed

        self.assert_refused(self.head(self.repository(), remove_name), "would break it")

    def test_refuses_an_event_field_removed_from_a_stable_release(self):
        def remove_connection(changed):
            event = changed["components"]["schemas"]["EventDelivery"]
            del event["properties"]["connected_account_id"]
            event["required"].remove("connected_account_id")
            return changed

        result = self.head(self.repository(), remove_connection)
        self.assert_refused(result, "would break it")
        self.assertIn("/webhooks/events", result.stdout + result.stderr)

    def test_refuses_a_changed_golden_transcript_of_a_stable_release(self):
        result = self.head(self.repository(), files={GOLDEN: '{"steps": [{"changed": true}]}\n'})
        self.assert_refused(result, "golden transcript")

    def test_refuses_a_stable_release_made_a_preview(self):
        result = self.head(self.repository(), lambda changed: spec("preview"))
        self.assert_refused(result, "stays stable")

    def test_refuses_a_stable_release_removed_without_the_sunset_label(self):
        repository = self.repository()
        repository.write({SPEC: None})
        repository.commit("head")
        self.assert_refused(repository.guard(), "api-sunset")

    def test_refuses_a_base_it_cannot_read(self):
        repository = self.repository()
        result = repository.guard(base="origin/no-such-branch")
        self.assertEqual(result.returncode, 2, result.stdout + result.stderr)

    # What passes.

    def test_passes_additions_to_a_stable_release(self):
        def add(changed):
            changed["components"]["schemas"]["Thing"]["properties"]["label"] = {"type": "string"}
            changed["components"]["schemas"]["EventDelivery"]["properties"]["attempt"] = {"type": "integer"}
            changed["paths"]["/v1/notion"] = {
                "get": {"operationId": "notion", "responses": {"200": {"description": "ok"}}}
            }
            return changed

        new_transcript = {f"api-reference/golden/{RELEASE}/notion.json": '{"steps": []}\n'}
        self.assert_passed(self.head(self.repository(), add, files=new_transcript))

    def sunset_repository(self, config=True):
        """anubis stable, and bastet out after it: anubis's sunset is bastet's date plus 730 days."""
        directory = tempfile.TemporaryDirectory()
        self.addCleanup(directory.cleanup)
        next_spec = f"api-reference/versions/{NEXT}.json"
        repository = Repository(
            directory.name,
            {SPEC: spec("stable"), next_spec: spec("preview", NEXT), GOLDEN: '{"steps": []}\n'},
            config=config,
        )
        repository.write({SPEC: None, GOLDEN: None})
        repository.commit("head")
        return repository

    def test_passes_a_stable_release_removed_past_its_sunset_with_the_label(self):
        self.assert_passed(self.sunset_repository().guard(sunset=True, today="2029-03-01"))

    def test_refuses_a_release_removed_before_its_sunset_even_with_the_label(self):
        result = self.sunset_repository().guard(sunset=True, today="2029-02-27")
        self.assert_refused(result, "advertise a sunset of 2029-02-28")

    def test_holds_a_removal_to_the_base_s_window_when_the_head_shortens_it(self):
        repository = self.sunset_repository()
        repository.write({"config.example.toml": "[api]\nsupport_days = 1\n"})
        repository.commit("shorten the window")
        result = repository.guard(sunset=True, today="2027-03-05")
        self.assert_refused(result, "advertise a sunset of 2029-02-28")

    def test_holds_a_removal_to_a_deployment_s_longer_window(self):
        repository = self.sunset_repository()
        environment = {"OSIRIS__API__SUPPORT_DAYS": "1000"}
        result = repository.guard(sunset=True, today="2029-03-01", extra=environment)
        self.assert_refused(result, "advertise a sunset of 2029-11-25")

    def test_refuses_a_removal_whose_window_is_unknown(self):
        self.assert_refused(
            self.sunset_repository(config=False).guard(sunset=True, today="2100-01-01"),
            "sunset is unknown",
        )

    def test_refuses_the_newest_release_removed_even_with_the_label(self):
        repository = self.repository()
        repository.write({SPEC: None, GOLDEN: None})
        repository.commit("head")
        self.assert_refused(repository.guard(sunset=True, today="2100-01-01"), "no sunset yet")

    def test_passes_a_breaking_change_to_a_preview(self):
        repository = self.repository(stability="preview")

        def remove_name(changed):
            changed["info"]["x-osiris-stability"] = "preview"
            del changed["components"]["schemas"]["Thing"]["properties"]["name"]
            changed["components"]["schemas"]["Thing"]["required"].remove("name")
            return changed

        self.assert_passed(self.head(repository, remove_name, files={GOLDEN: '{"steps": [1]}\n'}))

    def test_passes_a_base_with_no_releases_yet(self):
        directory = tempfile.TemporaryDirectory()
        self.addCleanup(directory.cleanup)
        repository = Repository(directory.name, {"README.md": "osiris\n"})
        repository.write({SPEC: spec("stable")})
        repository.commit("head")
        self.assert_passed(repository.guard())


if __name__ == "__main__":
    unittest.main()
