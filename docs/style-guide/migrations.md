# Migrations

- Plain SQL in `migrations/`, reversible (`just migration <name>` makes the `up` and `down`).
  CI applies every one, reverts every one, and applies them again.
- Applied by the migrate job, once per deploy, before the new revision takes traffic. Never at
  startup.
- Reference values are rows (`provider`, `connection_status`), not Postgres enums, so adding one
  is an insert.
- Every query is `query!`/`query_as!`, checked at compile time against the schema; `.sqlx/` is
  committed so CI and the image build without a database. After a schema or query change:
  `just sqlx-prepare`.
