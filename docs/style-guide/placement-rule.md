# Placement rule

Where a unit of code lives: a module, a helper, a type, a mock, a data factory, a third-party
wrapper. The same rule applies to all of them, in every crate.

1. **Used by one consumer: colocate it.** It is a sibling file of its consumer, or inside the
   consumer's folder.
2. **Used by more than one unrelated consumer: it lives in the crate below** (`osiris_common` for
   errors, secrets and ids; `osiris_interfaces` for what providers share), or in `testing/` for test
   setup.
3. **A helper used only by its parent nests inside the parent**, recursively. A helper of
   `use_cases/lib/refresh/` lives in that folder; one used only by that helper nests inside it.
4. **A module is `foo.rs` plus `foo/`**, never `mod.rs` (`clippy::mod_module_files`). The module
   file declares its children and re-exports; it holds no logic. The folder holds:
   - the module's units, one per file;
   - `tests.rs` or `tests/`, declared `#[cfg(test)] mod tests;`;
   - `mocks.rs` and `mocks/`, the module's mocks and data factories, declared
     `#[cfg(any(test, feature = "testing"))] pub mod mocks;` when other crates' tests use them.
5. **A unit with nothing to share a folder with is a module of its own**: `profile.rs` and
   `profile/`, not one file holding everything.
6. **A collection holds its members directly**: `use_cases/cases/` (one use case each),
   `use_cases/lib/<domain>/`, `routes/<domain>/` (one route each), `migrations/`.

An operation, a route, a use case, a type group or an adapter is one file.

## Domains in every layer

A domain has the same name in each layer it reaches. `connected_accounts` is
`routes/connected_accounts/`, `use_cases/cases/*_connected_account*.rs`,
`use_cases/lib/connected_accounts/` (its database operations) and
`osiris_api_models/src/connected_accounts.rs`.

Each provider is a domain in every layer, under its own id: `osiris_providers/src/<id>/`,
`osiris_api_models/src/actions/<id>.rs`, `use_cases/cases/<id>/`, `routes/actions/<id>/`,
`services/api/tests/<id>/`, `testing/<id>.rs`, `scripts/actions/<id>/` and
`docs/providers/<id>.md`. **A file outside a provider's folders never names a provider**, in its
path, its code, its comments or its tests; the only line that may is a module declaration
(`pub mod <id>;`). Lists of providers are written by `build.rs` from the folders, shared code finds
a provider through `Providers`, and shared tests use the fake provider. `just placement-check`
fails a build that breaks this ([add a provider](../add_provider.md)). It leaves out, each with
its reason in `scripts/placement-check.py`:

- generated artifacts: `api-reference/`, `.sqlx/`, the lockfiles and the client's generated code;
- the init migration, already applied, which seeded the first providers' rows;
- the hosting platform's own files and names, which a provider may share: `.github/`,
  `.gitleaksignore`, `deny.toml`, the client's release script and its tests
  (`clients/typescript/publish-release.mjs` and `publish-release.test.mjs`, which call the
  platform's API and package registry), and on any line `.github/` paths, `github.com` and
  `github.io` links and CI variables such as `GITHUB_STEP_SUMMARY`. Shared code and tests use none
  of them as fixtures.
