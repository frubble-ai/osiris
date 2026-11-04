# Osiris

Frubble's integrations service. Organizations connect their GitHub, Linear and Slack to it;
it runs the OAuth flow, keeps their tokens encrypted and fresh, and answers reads with them,
over an HTTP API. Its clients (Frubble's app API) never hold a provider token.

## Running it locally

Needs Rust (the toolchain in `rust-toolchain.toml` installs itself), Docker, and
`brew install just sqlx-cli cargo-nextest cargo-deny bacon`.

```sh
cp .env.example .env
just up        # Postgres on localhost:5433, migrations applied
just dev       # the service on http://localhost:8090, rebuilt on change
curl localhost:8090/health
```

`just` lists every command. `just check` runs what CI runs: format, clippy, cargo-deny,
the tests, and the sqlx offline check.

## Database and migrations

Osiris owns its database: nothing else reads or writes it. Migrations are plain SQL in
`migrations/`, made with `just migration <name>` (reversible: an `up` and a `down` file) and
compiled into the binary. They are never applied at startup: the deploy runs `osiris migrate`
once, as its own job, before the new revision takes traffic.

Queries are checked at compile time against the schema. CI and the image build have no
database, so they check against `.sqlx/`: after changing a query, run `just sqlx-prepare` and
commit `.sqlx/`.

## The image

`docker build .` makes one image with both commands: `osiris serve` (the default) and
`osiris migrate`.
