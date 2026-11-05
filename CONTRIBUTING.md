# Contributing

- **Branches**: from `dev`, named `feat/…`, `fix/…`, `docs/…`. Never push to `dev`, `stg` or
  `main`: every change arrives through a pull request.
- **Commits**: [Conventional Commits](https://www.conventionalcommits.org), checked by cocogitto
  (`cog check`). The PR title is the squash commit, so it follows them too.
- **Pull requests**: against `dev`, every section of the template filled; CI checks it.
- **Before pushing**: `just check` (format, clippy with warnings as errors, cargo-deny, spelling,
  `.sqlx/`, the tests, the OpenAPI spec). A migration also runs `just migrations-check`.
- **Tests** are part of the change: see [test organization](docs/style-guide/test-organization.md) and [testing patterns](docs/style-guide/testing-patterns.md).
