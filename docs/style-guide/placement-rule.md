# Universal Placement Rule

Where a unit of code lives: a module, a helper, a type, a mock, a data factory, a third-party wrapper. The same rule applies to every one of them, in every crate.

1. **Used by only one consumer → colocate it.** It lives as a sibling file of its consumer, or inside the consumer's folder. Never more than one level up.
2. **Used by more than one unrelated consumer → it lives in the crate the layer below provides** (`osiris_common` for errors, secrets and ids; `osiris_interfaces` for what providers share), or in `testing/` for test setup.
3. **A helper used only by its parent nests inside the parent, recursively.** A helper of `use_cases/lib/refresh/` lives in that folder, not in `lib/shared/`; one used only by that helper nests inside it again.
4. **A module is `foo.rs` plus `foo/`, never `mod.rs`** (`clippy::mod_module_files`). The module file declares its children and re-exports; it holds no logic. The folder holds:
   - the module's units, one per file;
   - `tests.rs` or `tests/`, the module's tests, declared `#[cfg(test)] mod tests;`;
   - `mocks.rs` and `mocks/`, the module's mocks and data factories, declared `#[cfg(any(test, feature = "testing"))] pub mod mocks;` when other crates' tests use them.
5. **A unit with nothing to share a folder with is a module of its own**: `profile.rs` and `profile/`, not one `profile.rs` holding everything. It gets its folder the moment a second unit or a test appears.
6. **Collections are the exception.** A folder whose files are all members of one kind holds them directly: `use_cases/cases/` (one use case each), `use_cases/lib/<domain>/`, `routes/<domain>/` (one route each), `migrations/`.

## Domains in every layer

A domain has the same name in each layer it reaches: `connected_accounts` is `routes/connected_accounts/`, `use_cases/cases/*_connected_account*.rs`, `osiris_storage/src/connected_accounts/` and `osiris_api_models/src/connected_accounts.rs`. Each provider is its own domain in every layer: `osiris_providers/src/github/`, `osiris_api_models/src/actions/github.rs`, `routes/actions/github/`. Beyond those folders, only the lists that register providers name one ([add a provider](../add_provider.md)).

## How to decide

- One consumer → colocate.
- More than one → the crate below, or `testing/`.
- A helper with one consumer → inside its parent.

The lints check what a linter can see (`mod_module_files`, the workspace lints). Whether a unit has one consumer or several is a call the author and the reviewer make.

## One unit per file

An operation, a route, a use case, a type group or an adapter is a file. A file does one thing; a second thing is a second file.
