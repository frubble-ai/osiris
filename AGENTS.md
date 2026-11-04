# AGENTS.md

For coding agents working in this repository. Start with README.md.

- Rust, axum on tokio, sqlx on Postgres. One binary: `osiris serve` and `osiris migrate`.
- Osiris owns its database. Migrations are SQL files in `migrations/` (`just migration <name>`),
  applied by `osiris migrate`, never at startup. After changing a query, `just sqlx-prepare`
  and commit `.sqlx/`.
- Each provider (github, linear, slack) is its own module, the same way in every part of the code.
- Never log or return a token or a client secret. Tokens and secrets are encrypted at rest.
- `just check` must pass before a commit: format, clippy (warnings are errors), cargo-deny,
  the tests, and the sqlx offline check.
- Branch from `dev` and open pull requests against `dev`.
- Commit messages follow Conventional Commits.
