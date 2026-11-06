# Migrations

- Plain SQL in `migrations/`, reversible (`just migration <name>` makes the `up` and `down`).
  CI applies every one, reverts every one, and applies them again.
- Applied by the migrate job, once per deploy, before the new revision takes traffic. Never at
  startup.
- Rewriting a migration that a database has already applied is not a `down`. After that deploy,
  [`db-reset.yml`](../../.github/workflows/db-reset.yml) drops the tables and enums this database
  user owns in `public` and runs the migrations from nothing
  ([configuration](../configuration.md#resetting-the-database)).
- Reference values are rows (`provider`, `connection_status`), not Postgres enums, so adding one
  is an insert.
- Every query is `query!`/`query_as!`, checked at compile time against the schema; `.sqlx/` is
  committed so CI and the image build without a database. After a schema or query change:
  `just sqlx-prepare`.
- A `DO` block and DDL (`CREATE`, `ALTER`, `DROP`) are `sqlx::query`. `query!` prepares the
  statement, and PostgreSQL refuses `PREPARE` of both. An identifier cannot be a bind parameter
  either. The reset's drop is the `DO` block in `crates/osiris_server/src/db/drop_owned.rs`. The test that
  creates a role, to check the owner filter, is the DDL.
