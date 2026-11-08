"""Tests of placement-check.py."""
import importlib.util, os, unittest

spec = importlib.util.spec_from_file_location(
    "placement_check", os.path.join(os.path.dirname(__file__), "placement-check.py")
)
placement_check = importlib.util.module_from_spec(spec)
spec.loader.exec_module(placement_check)

IDS = ["acme", "widgets"]


class Violations(unittest.TestCase):
    def test_a_file_in_its_provider_s_folder_may_name_it(self):
        self.assertEqual(placement_check.violations("crates/x/src/acme/call.rs", "Acme's API", IDS), [])
        self.assertEqual(placement_check.violations("docs/providers/acme.md", "# Acme", IDS), [])

    def test_a_shared_file_naming_a_provider_is_refused_with_its_line(self):
        found = placement_check.violations("crates/x/src/shared.rs", "ok\n// like Acme does\n", IDS)
        self.assertEqual(found, [(2, "// like Acme does")])

    def test_a_module_declaration_is_allowed(self):
        text = "pub mod acme;\nmod widgets;\npub(crate) mod acme;\n"
        self.assertEqual(placement_check.violations("crates/x/src/lib.rs", text, IDS), [])

    def test_a_path_naming_a_provider_outside_its_folder_is_refused(self):
        found = placement_check.violations("crates/x/tests/acme_actions.rs", "", IDS)
        self.assertEqual(len(found), 1)

    def test_a_reference_to_the_hosting_platform_is_not_a_provider(self):
        ids = ["github"]
        text = "see .github/workflows/ci.yml\nhttps://github.com/org/repo\nGITHUB_STEP_SUMMARY\n"
        self.assertEqual(placement_check.violations("docs/ci.md", text, ids), [])
        self.assertEqual(len(placement_check.violations("docs/ci.md", "GitHub's API", ids)), 1)

    def test_an_exempt_file_is_not_checked(self):
        self.assertEqual(placement_check.violations(".github/workflows/ci.yml", "acme", IDS), [])


if __name__ == "__main__":
    unittest.main()
