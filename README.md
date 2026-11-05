# Osiris

Frubble's integrations service. A client (Frubble's app API) registers its OAuth apps with
osiris, sends its users through a provider's consent, and then runs typed actions with their
connections. Osiris holds the tokens (encrypted, per tenant), keeps them fresh with scheduled
refresh tasks, and tells the client when a connection needs reconnecting. The client never holds a
provider token.

It runs as two request-driven Cloud Run services, `serve` (public) and `worker` (internal), and a
`migrate` job. Work at a set time is a Cloud Task.

Providers: GitHub (through a GitHub App), Linear, Slack.

## Running it locally

Needs Rust (the toolchain in `rust-toolchain.toml` installs itself), Docker, and
`brew install just sqlx-cli cargo-nextest cargo-deny typos-cli cocogitto`.

```sh
cp .env.example .env
just up        # Postgres over TLS (5433, with a local CA in .local/tls), the Pub/Sub emulator
               # (8086) with the events topic, then the migrate process
just dev       # OSIRIS_PROCESS=serve on PORT=8090 and OSIRIS_PROCESS=worker on PORT=8091
curl localhost:8090/health
```

Locally, osiris reads the same `DB_*` variables with TLS verified against `DB_CA_PEM`, and the
process is chosen by `OSIRIS_PROCESS` ([docs/configuration.md](docs/configuration.md)).

Then, with the CLI (`cargo install --path crates/osiris_cli`, or `cargo run -p osiris_cli --`):

```sh
osiris login --url http://localhost:8090      # reads the keys and the topic from .env
OSIRIS_CLIENT_SECRET=… osiris auth-configs create --provider github --client-id Iv1.…
osiris connect github --user org_1            # opens the provider's consent
osiris connections list
osiris connections refresh ca_…               # refresh a connection's tokens now (admin key)
osiris actions run github.list_repositories --connection ca_…
osiris listen --forward-to http://localhost:8081/push/osiris-events
osiris trigger connection.expired --connection ca_…
```

`just` lists every command; `just check` runs what CI runs.

## Where to read next

- [docs/architecture.md](docs/architecture.md): the crates, the processes, and how a request,
  a refresh and an event flow.
- [docs/configuration.md](docs/configuration.md): every setting and environment variable.
- [docs/api-versioning.md](docs/api-versioning.md): `Osiris-Version`, releases, breaking changes.
- [docs/add_provider.md](docs/add_provider.md): adding a provider or an action.
- [docs/style-guide/index.md](docs/style-guide/index.md): how code here is written.
- [CONTRIBUTING.md](CONTRIBUTING.md): branches, commits, pull requests.
- The API itself: [api-reference/openapi.json](api-reference/openapi.json), generated from the
  code (`just openapi`).
