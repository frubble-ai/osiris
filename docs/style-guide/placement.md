# Placement

- **Domains in every layer.** A domain (auth configs, connected accounts, task outcomes, each
  provider) has the same name in each layer it reaches: `routes/connected_accounts/` ↔
  `core/connected_accounts/` ↔ `storage/connected_accounts/` ↔ `api_models/connected_accounts.rs`.
  Each provider is its own folder in every layer it reaches (`osiris_providers/src/github/` ↔
  `core/actions/github/` ↔ `routes/actions/github/`); beyond those folders, only the lists that
  register providers name one.
- **One unit per file.** An operation, a route, an action or a type group is a file. The module
  file (`foo.rs`) only declares its children and re-exports: `pub mod create; pub use …`.
- **`foo.rs` + `foo/`, never `mod.rs`** (`clippy::mod_module_files`).
- **Shared code** within a domain sits beside it (`rows/`, `lib.rs`); across domains, in the
  crate the layer below provides (`osiris_common`, `osiris_interfaces`).
