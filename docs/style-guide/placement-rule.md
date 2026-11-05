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
`osiris_api_models/src/connected_accounts.rs`. Each provider is a domain in every layer:
`osiris_providers/src/github/`, `osiris_api_models/src/actions/github.rs`,
`routes/actions/github/`. Beyond those folders, only the lists that register providers name one
([add a provider](../add_provider.md)).
