# API versioning

As Stripe does it. The path is `/v1`, a generation of the API's ground rules; the version is a
date and a named release, sent as `Osiris-Version`, e.g. `2026-11-04.anubis`.

- A request naming no version is served at the API key's pinned version (`[api] pinned_version`).
  One naming a version osiris doesn't serve is refused (`OS_07`). Every answer names the version
  it was served at.
- Additions (a field, an endpoint, an action) need no new version.
- A breaking change comes with a new version, and with it a version change: a module that turns
  requests from the old version into the current model and answers back into the old shape, so
  clients pinned to it keep working. The code has one current model; old versions are only these
  transforms.
- Named releases follow the Egyptian pantheon, alphabetically: anubis, bastet, …
- Each version's changes for clients go in [`changelog/`](changelog/).

Ground rules of `/v1`: an `Idempotency-Key` on any POST or DELETE runs it once; lists are cursor
batches (`after`/`before`, `next_cursor`/`previous_cursor`); events are stateless.
