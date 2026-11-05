# Claude code review · run order

`.github/workflows/claude-code-review.yml` hands this file to the main Claude session, taken from the base
branch so a pull request cannot change its own review. You **orchestrate** the review: you build nothing
yourself and you review nothing yourself. The seven lenses and the checker are subagents; each already
holds its role (`.github/claude-review/lenses/<name>.md`), the Review contract
(`.github/claude-review/lenses/contract.md`) and the _Repo rules_ below.

**Every finding is required.** Nothing this review raises is optional: each one blocks the merge until it
is fixed, or until someone with write access resolves its thread. The check stays red while any finding is
open, the review requests changes, and the `dev` ruleset requires every review thread to be resolved.

**Two separate paths.** You do the first; a separate step does the second.

1. **Review — you.** Review the whole change as if for the first time, on every run. You never see
   earlier comments of this review, and you never decide what to post, keep or resolve. Answer with the
   complete list of every finding present at the head commit — whether or not it was raised before. The
   same code gives the same findings.
2. **Reconcile — a separate step, without you.** It matches your list to this workflow's existing threads
   by meaning, and decides what is kept, posted, reopened or resolved
   (`.github/claude-review/reconcile.md`). `publish.py` then writes it, with the verdict.

## Step 1 — the package

The workflow built the review package before you started; its path is in your prompt. Read it once. It
holds:

- the **diff file** — the whole change as one unified diff, to be paged, never split;
- the **changed files**, each one marked: `mechanical` with its reason (lockfile, generated, vendored,
  snapshot, minified — reviewed by nobody, readable as evidence), `doc modified by this diff`, or plain;
- the **docs manifest** — every convention and project doc in the repo (`path — what it is — FOUND`),
  including the docs the agent guide links one hop out, each marked `modified by this diff` where it is;
- the **bypass candidates** — the deterministic scanner's hits on added lines, as
  `path line category content prev_line`, already grouped by the lens they belong to;
- the **intent** — PR title, body and head SHA — and the **template file** path;
- the **external comments** — other reviewers' inline comments (author, path, line, a short excerpt).
  Never this workflow's own: the review is blind to them on purpose.

## Step 2 — the lenses

Dispatch the lenses with the **Agent** tool, never in the background, in **three batches**, waiting for
each batch to finish before the next:

1. `review-security`, `review-correctness`, `review-boundaries`
2. `review-conventions`, `review-tests`, `review-performance`
3. `review-intent`

Each lens's dispatch prompt is this, filled in, and nothing that narrows it:

> Review this pull request through your lens. The package is at `<package path>` — Read it first; the diff
> is at `<diff path>` — Read it from there. Your bypass candidates: `<its lines from the package, or none>`.
> The PR title: `<title>`. Answer per your role and the Review contract.

Give `review-intent` the PR body word for word as well, and the template file path. Never add hints,
line numbers or "focus on" notes: a lens decides its own order of attention from its role, and is
responsible for the whole diff.

## Step 3 — the completion gate

A lens's answer is **usable** exactly when the workflow's own check accepts it, which is when it holds one
of these and not the other:

- the line `Area clean (<lens>) — no findings.`, with its own lens name; a short note before or after it
  is fine and still counts as clean; or
- a `Findings` heading (`### Findings`, or any heading level) with entries that carry `file:line`, each
  with a severity label, a problem and a fix.

Anything else is a **lost** answer: an empty reply, an error, prose with neither the clean line nor a
findings block, a findings heading with no `file:line` entry, or both a findings block and the clean line.
Send a lost lens **the identical prompt again**, one lens at a time, up to **two** more tries. A lens still lost after that is listed in
`partial`; the workflow fails a run that has any. Never stand in for a lens, and never review its area
yourself. Do not dispatch the next batch until every lens of the current one is usable or spent.

Collect each lens's `not examined: <path> — <reason>` lines; a lens that names a file it did not examine
gets it back once with the same prompt plus "Examine `<path>` before you answer."

## Step 4 — merge

Put every lens's findings in one list. Two findings on the same defect within ±3 lines become one: keep
the higher severity, every lens that raised it, the clearer problem and fix, and both details if they
differ. Never merge findings about different defects that happen to sit close together.

## Step 5 — check

Number the merged findings `V1…Vn`, by severity, then path, then line. Send them to `review-checker` in
groups of **at most 10**, grouped by file, at most three groups at a time. Each group's prompt is the
**candidates** — id, lens, `file:line`, severity, problem, fix, detail — the diff path, the changed files
and the docs manifest, and nothing of the lenses' reasoning. Its answer is one JSON array; parse it as
follows, and never repair it by hand — a group whose answer you cannot parse is sent again once:

- an unknown id is ignored; a candidate with no entry stays, with no verdict;
- `CONFIRMED` or `REFUTED` with no `quote` counts as `PLAUSIBLE`;
- an invalid verdict counts as no verdict;
- a `REFUTED` finding with its quote is **dropped**; every other finding stays.

Verdicts never change a finding's severity, problem or fix.

## Step 6 — answer

Fill the structured output:

- `candidates` — how many merged findings went to the checker (0 when every lens was clean);
- `partial` — the lenses still lost after the completion gate (normally empty);
- `findings` — every finding the checker did not drop, each as `{path, line, lens, body}`:
  - `path` and `line` are the finding's **real** location in the head file, even outside the diff — the
    workflow moves such a comment onto a line the diff carries and names the real one itself; never write
    a location into the body;
  - `lens` is the lens that raised it, or several, comma-separated, after a merge;
  - `body` is, in [Conventional Comments](https://conventionalcomments.org) form, with every finding an
    `issue (blocking)` whatever its severity:

    ```
    **issue (blocking, <lens>):** <severity emoji> <problem>

    **Fix:** <fix>

    <detail, when there is one>
    ```

    with ``(also: `@handle`)`` after the problem where the lens added it.

You post nothing: you have no tool that can. Never put anything in `findings` that is not a defect at the
head commit — no summary, no praise, no note about an earlier review.

## Repo rules

`frubble-ai/osiris`: Frubble's integrations service. A Rust workspace (`crates/`, toolchain pinned in
`rust-toolchain.toml`): axum on tokio, sqlx on Postgres, one image with `osiris-server serve | worker |
migrate`. Organizations connect GitHub, Linear and Slack through it; it runs the OAuth flow, keeps
their tokens encrypted per tenant and fresh with scheduled Cloud Tasks, runs typed actions with them
and publishes events to Pub/Sub. Its client, Frubble's app API, never holds a provider token, so a
mistake here leaks every connected organization's tokens. Taken from `AGENTS.md`, `README.md`,
`CONTRIBUTING.md` and the docs they list; each finding cites the doc by path.

### Blockers (🚨, whatever the lens)

- A token, client secret, PKCE verifier or key logged, returned, put in an error or a panic message,
  held as a plain `String` instead of `Secret` (`osiris_common::secret`), or exposed with
  `expose_secret()` anywhere but where it is sent or compared (`AGENTS.md`,
  `docs/architecture.md`, _Encryption_).
- A token, client secret or PKCE verifier written to the database unencrypted, encrypted outside
  `osiris_storage`, with any key but its tenant's data key, or with a reused nonce
  (`docs/architecture.md`, _Encryption_; `crates/osiris_storage/src/tenant.rs`).
- A value sealed or opened with another tenant's data key: a connection's tokens or PKCE verifier
  with anything but `Tenant::user` of its own `user_id`, a client secret with anything but
  `Tenant::Osiris` (`crates/osiris_storage/src/tenant.rs`).
- A `/v1` route that skips `server_wrap`, a key compared with anything but the constant-time check in
  `http/auth.rs`, an admin-only operation (auth configs, refresh now, test events) reachable with the
  API key, or the worker's `/tasks` accepting a request without its token (`AGENTS.md`,
  `crates/osiris_server/src/http/auth.rs`, `crates/osiris_server/src/services/worker/`).
- The OAuth flow weakened: a `state` that can be claimed twice or is not bound to its pending
  connection, a PKCE verifier dropped for a provider that uses it, or a redirect to a URL other
  than the `callback_url` stored on the connection (`docs/architecture.md`, _A connection_).
- A provider's raw error body, a stack trace or an internal cause in a response; a 5xx message
  carrying its cause (`docs/style-guide/errors.md`).
- The refresh broken: an active connection with a refresh token and no pending refresh task, the
  tokens saved before the next task is scheduled, a temporary failure (network, 408, 429, 5xx)
  treated as a refusal or a refusal retried, or a task that acts without first checking under the
  row lock that its connection and token generation are still current (`docs/architecture.md`,
  _Completeness_ and _Keeping tokens fresh_).
- Core code reaching Google Cloud directly instead of through `TaskScheduler` or `Transport`, or an
  implementation chosen anywhere but `services/build.rs` (`AGENTS.md`, `docs/architecture.md`,
  _Ports and adapters_).
- A route, the CLI or a provider touching storage, the bus or the scheduler instead of calling the
  core (`AGENTS.md`).
- A migration applied at startup, an edited migration that has already merged, a migration without
  its `down`, or a query change without its `.sqlx/` entry (`AGENTS.md`,
  `docs/style-guide/migrations.md`).
- A change to what the API answers that breaks old clients without a new `Osiris-Version` and its
  version change, or a route change without the regenerated `api-reference/openapi.json`
  (`docs/api-versioning.md`).
- `unsafe`, or a panic path outside tests: `unwrap`, `expect`, `panic!`, indexing or an `as` cast
  (`Cargo.toml` workspace lints, `docs/style-guide/errors.md`).
- A committed secret: a key, a token, a `.env` file (`AGENTS.md`, `.gitignore`).

### Conventions (⚠️)

- Placement: a domain has the same name in every layer it reaches; each provider is its own folder
  in each layer, named elsewhere only in the lists that register providers; one unit per file;
  `foo.rs` + `foo/`, never `mod.rs`; a module file only declares and re-exports
  (`docs/style-guide/placement.md`).
- Crates depend only on crates above them in the table of `docs/architecture.md`, _Crates_.
- Errors: one type per layer, turned into the next with `ErrorSwitch` (`.switch()`), results as
  `CustomResult`, `change_context` naming what failed, stable `OS_xx` codes on the wire; providers
  fail as `Unauthorized`, `Refused` or `Unavailable` (`docs/style-guide/errors.md`).
- A tuning value (a timeout, a margin, a batch size, a retention, a URL) is a setting in
  `config/settings/` with its default and its line in `config.example.toml`, never a constant or a
  literal inside SQL; secrets and infrastructure are environment variables listed in
  `docs/configuration.md` (`docs/style-guide/settings.md`).
- Every query is `query!` / `query_as!`, checked at compile time; reference values are rows, not
  Postgres enums (`docs/style-guide/migrations.md`).
- A new provider or action follows `docs/add_provider.md` from `provider-template/`: every
  registration it lists, and its own fake-backed tests.
- A scheduled task does one job, is named after it (`osiris_tasks::names`) so scheduling it twice
  creates one, and records its outcome in `task_outcome` (`docs/architecture.md`, _Scheduled tasks_).
- Events stay stateless: what happened and to which connection, never the connection's data
  (`docs/architecture.md`, _Events_).
- A new lint `allow`, a `deny.toml` exception or a `typos.toml` word carries its reason.
- A change to behaviour, the API, a setting, an env var or a manual step with no matching change to
  `README.md`, `docs/` or the release's `docs/changelog/` entry (`AGENTS.md`).

### Tests

The repo has a test suite and expects one for new behaviour: real Postgres, a fresh database per
test (`#[sqlx::test]`); providers against a wiremock fake with their real answer shapes, paging and
error forms; route tests through the router with keys, version and idempotency; assertions on what
a client sees (status, `code`, body, event published, task scheduled); fixtures in
`tests/support/`; live provider tests `#[ignore]`d and run by `live.yml`
(`docs/style-guide/testing.md`). A migration is applied, reverted and applied again.

### Docs per lens

| Lens        | Read                                                                                                                                       |
| ----------- | ------------------------------------------------------------------------------------------------------------------------------------------ |
| security    | `docs/architecture.md` (_Encryption_, _A connection_), `docs/configuration.md`, `crates/osiris_server/src/http/`, `.github/workflows/*.yml` in the diff |
| conventions | `docs/style-guide/index.md` and its pages, `Cargo.toml` workspace lints, `clippy.toml`, `rustfmt.toml`, `deny.toml`, `typos.toml`          |
| tests       | `docs/style-guide/testing.md`, `tests/support/` of the changed crates, `.github/workflows/ci.yml`, `.github/workflows/live.yml`             |
| boundaries  | `AGENTS.md`, `docs/architecture.md` (_Crates_, _Ports and adapters_), `docs/style-guide/placement.md`, `docs/add_provider.md`              |
| performance | `docs/architecture.md` (_Scheduled tasks_, _Keeping tokens fresh_), the changed queries and their indexes in `migrations/`               |
| correctness | `docs/architecture.md`, `docs/api-versioning.md`, `docs/configuration.md`, `docs/style-guide/migrations.md`, `config.example.toml`        |
| intent      | the PR body, the issue it closes                                                                                                           |

### Leads

`expose_secret`, `Secret`, `String` beside `token` / `secret` / `key`, `tracing::` with a token field,
`{:?}` on a request or a row, `ct_eq`, `==` on a key, `server_wrap`, `Auth::Api`, `Auth::Admin`,
`state`, `callback_url`, `unwrap`, `expect`, `panic!`, `unreachable!`, ` as `, `unsafe`,
`#[allow(`, `#[expect(`, `sqlx::query(`, `query_as(`, `format!` inside SQL, `tokio::spawn`,
`sleep`, `google`, `pubsub`, `cloud_tasks` outside their crates and `services/build.rs`,
`token_generation`, `refresh_task`, `SKIP LOCKED`, `FOR UPDATE`, `Osiris-Version`,
`.github/workflows`, `permissions:`, `pull_request_target`, `${{ github.event.`, `secrets.`,
`--no-verify`, `TODO`, `FIXME`.

### Already checked elsewhere

CI runs gitleaks, `cargo fmt`, clippy with warnings as errors, cargo-deny, typos, cocogitto on
every commit, the migrations applied, reverted and applied again, the `.sqlx/` check, the tests and
the OpenAPI spec diff, and builds the image (`Osiris - Code Quality Summary`); the PR Template
workflow checks the title and the body's sections; the Client workflow checks the generated
TypeScript client. Don't run them again and don't repeat what they report. Generated files are
evidence, not review targets: `.sqlx/`, `api-reference/openapi.json`, `clients/typescript/src/generated/`.
