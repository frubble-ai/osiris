# API versioning

The path is `/v1`, a generation of the API's ground rules: keys, `Idempotency-Key`, cursor batches
and stateless events. It changes only if those rules change, and that is not expected.

The version is a **release**: a date and a god of the Egyptian pantheon, in alphabetical order
(`2026-11-04.anubis`, then bastet, …). A request names it in `Osiris-Version`. One naming none is
served at the API key's pinned release (`[api] pinned_version`), and one naming a release osiris
doesn't serve is refused (`OS_07`). Every answer names the release it was served at
(`Osiris-Version`) and whether it is a preview (`Osiris-Stability`). Events are written in the
pinned release and say so in `osiris_version`.

The release name changes only when something breaks. Adding a provider, an action, an endpoint, a
field or an event type never changes it.

## What breaks

The test: could a client written for the release, our own generated client included, stop working
without changing its code? The generated client ignores fields it doesn't know, and it refuses a
value outside an enum in an answer.

| Change | Breaks |
| --- | --- |
| A new provider, action, endpoint, or optional request field or query parameter | no |
| A new field in an answer or an event | no |
| A new value of an open string (an event `type`, an error `code`, a `provider`, a `status`) | no |
| A new answer status the spec documents, such as a new 4xx for a new failure | no |
| Accepting more: a looser validation, a new allowed input value | no |
| Rewording the docs, a description, a summary | no |
| Removing or renaming a field, a path, an action, a provider or an event type | **yes** |
| Changing a field's type or format (a string to an object, an id's prefix, a time's format) | **yes** |
| A new required request field, or an optional one made required | **yes** |
| A new value in a closed enum of an answer | **yes** |
| A field always present becoming null or absent | **yes** |
| A different default, status, `OS_xx` meaning, ordering, idempotency or pagination behaviour | **yes** |
| A different shape for an existing action's answer | **yes** |

### Rules that keep growth from breaking

- **Values that grow are open strings, never closed enums in answers.** The spec says
  `type: string` and the description lists today's values. A client keeps working on a value it
  doesn't know and ignores an event type it doesn't handle. This covers an event's `type`, an
  error's `code`, `provider`, a connection's `status` and GitHub's `account_type` and
  `repository_selection`. An enum is only for input osiris reads, where a new value is an addition.
- **One action, one path, one schema.** A new provider brings new paths and touches none of the
  others.
- **A provider's quirks stay in its own actions' answers.** Shared shapes (`ConnectedAccount`,
  `AuthConfig`, `EventDelivery`) never gain a provider-specific required field.
- **A provider retiring an API we use** is answered with a documented error (`OS_04`, 424), never by
  removing the action within a release.

## Preview and stable

A release is a **preview** or **stable** (`Stability` in `osiris_versioning`).

- A **preview**'s contract may still change in any way. Its answers say `Osiris-Stability: preview`
  and its client is published only on the branch channels, never on latest.
- A **stable** release's contract only grows. Additions are made in place, and a breaking change
  waits for the next release. CI holds its spec to breaking nothing and its golden transcripts to
  never changing.
- Only the newest release can be a preview, and a stable release stays stable.

anubis is a preview while the app is its only client. It becomes stable in the pull request that
cuts it, before a client depends on it in production.

## How a release is served

The code has one model, the current release's. An older release exists only as **version changes**
(Stripe's and Intercom's version change modules, Cadwyn's migrations). Each release after the first
lists the changes it made in `RELEASES`, and each change (`VersionChange`) says how:

- `request` and `query` bring a request written for the release before up to this one;
- `response` takes an answer back to the release before;
- `event` takes an event back to the release before;
- `spec` takes the `OpenAPI` document back to the release before.

The version middleware (`http/version/`) resolves the release, brings the request up through every
change since it, runs the route, and takes the answer back down. The bus takes each event down to
the pinned release. `osiris_openapi` writes each release's spec, taken down the same way, to
`api-reference/versions/<release>.json`. A change only touches the routes, statuses and events it is
about. The idempotency middleware runs inside the version middleware, so it keys and stores the
current release's shape.

An answer at a superseded release carries `Deprecation` (RFC 9745, the next release's date) and
`Sunset` (RFC 8594, that date plus `[api] support_days`, 730 by default). The release's spec
records that sunset at the default window as `info.x-osiris-sunset`; it may only stay or move
later, and a removal is held to it. Past its sunset, a release
is still served until a pull request removes it.

## What holds it

| Check | Where | What it refuses |
| --- | --- | --- |
| Conformance | every API test (`testing/conformance`) | an answer whose status the spec doesn't document, or whose body doesn't match its schema; an event that doesn't match `EventDelivery` |
| Golden transcripts | `services/api/tests/golden` | a release no longer doing what was recorded for it: a field gone or changed, a status, a header, an event |
| Release rules | `osiris_versioning`'s tests, and `services/api/tests/releases.rs` for each spec and changelog | releases out of date, name or number order, a preview that isn't the newest, a later release without its changes, a current spec that doesn't name the current release |
| Specs match the routes | `just openapi-check`, CI | a committed spec that isn't what the code generates |
| Stable releases break nothing | `just contract-check`, CI's Contract job | an oasdiff error or warning in a stable release's spec, a stable release made a preview, removed without the label or before its sunset, a stable release's golden transcript changed |
| Schemathesis | `just schemathesis` (`scripts/schemathesis/`), CI's Tests job | a live osiris answering a generated request at any release and key with an undocumented status, content type or body, or a server error |
| Client release line | `npm run check`, the publish step | a client sending another release than its spec's, a major version that isn't its release's number (`info.x-osiris-release-number`), a preview published on latest |

Golden transcripts live in `api-reference/golden/<release>/`, with ids, times and random tokens
masked. A field osiris adds later passes; anything recorded that changes fails.
`OSIRIS_GOLDEN=record just test golden` records them again, which a preview may do and a stable
release may not.

## Processes

### An addition

A new provider, action, endpoint, field or event type: build it, run `just openapi`, record the new
golden transcripts if a scenario covers it, and commit. The release name, `RELEASES` and the
existing transcripts stay as they are. CI's Contract job confirms nothing stable broke.

### A breaking change

- **While the newest release is a preview**, make the change, run `just openapi`, record the
  transcripts again, and note it in the release's changelog.
- **When the newest release is stable**, the change belongs to the next release:
  1. If there is no next release yet, add one at the end of `RELEASES` as a preview, named with
     the next god, dated the day it is cut, and numbered one after the last release ever made.
     Point `CURRENT` (`osiris_versioning/src/releases.rs`) at it, and the client's
     `OSIRIS_VERSION` (`clients/typescript/src/index.ts`) to its name. Give it a `golden_at!` line
     in `services/api/tests/golden/releases.rs` and a changelog file, and move the client's major
     version to its number (`npm run check` says which).
  2. Change the code to the new shape.
  3. Write a version change under `osiris_versioning/src/changes/<release>/`, one file per
     change, and list it in the release's `changes`. It must give the older release its old
     requests, answers, events and spec back. Its `description` goes in the release's changelog
     word for word; a test checks it is there.
  4. Run `just openapi`, then record only the new release's transcripts:
     `OSIRIS_GOLDEN=record just test golden::releases::<god>` (its `golden_at!` module). Recording
     the older ones would write their new `Deprecation` and `Sunset` headers into transcripts that
     may not change. `just test golden` then replays every release, and the stable release's spec
     and transcripts must come out unchanged, which the Contract job checks.

Breaking changes collect in the next release until it is cut. Releases are cut on demand, not on a
schedule.

### Cutting a release

One pull request:
1. Set its `stability` to `Stability::Stable`.
2. Run `just openapi` and record its transcripts (`OSIRIS_GOLDEN=record just test
   golden::releases::<god>`).
3. Finish its changelog.
4. For the first release, move the client's major version from 0 to 1 (`npm run check` says
   which). A later release's client already carries its number.

From its date, the release before is deprecated, and its sunset is `support_days` later.

### Removing a release

Only the oldest release, only past its sunset, in a pull request labelled `api-sunset`. Releases
are removed oldest first, so the ones left keep consecutive numbers, which the release rules check.
The Contract job checks the label and that the day is past the next release's date plus the
longest support window clients were promised: the base branch's documented `support_days`
(`config.example.toml`), the pull request's, or `OSIRIS__API__SUPPORT_DAYS` when a deployment
sets its own (set it for the job too). With none of them known, the removal is refused. The
newest release has no sunset, so it is never removed.
1. Remove it from `RELEASES`, along with its `golden_at!` line, its spec and its transcripts.
2. The next release becomes the first: delete the version changes that only served the removed
   one, and empty that release's `changes`.

Clients still pinned to it get `OS_07`, so move the API key's `pinned_version` first.

Each release's changes for clients go in [`changelog/`](changelog/).
