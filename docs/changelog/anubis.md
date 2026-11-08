# 2026-11-04.anubis

The first release, a **preview**: its contract may still change until it is cut as stable
(`Osiris-Stability` says which).

- Auth configs: register, list, read, update and delete providers' apps (admin key).
- Connected accounts: open a consent, list (by user, provider, auth config, alias, status), read,
  delete (the grant is ended at the provider), and refresh now (admin key). Each connection reports
  `setup_complete`. A connection to a provider that issues refresh tokens is only `active` when its
  tokens and their scheduled refresh are both saved; otherwise it is `disabled`. `refresh_task` and
  `refresh_task_at` show the pending refresh.
- Actions: `github.list_installations`, `github.list_repositories`, `linear.list_teams`,
  `slack.list_channels`.
- Events: `connection.active`, `connection.expired`, `connection.revoked`; test events with
  `POST /v1/events/trigger` (admin key).
- Versions: every answer names its release in `Osiris-Version` and its stability in
  `Osiris-Stability`. An answer at a superseded release carries `Deprecation` and `Sunset`.
- Events carry `osiris_version`, also sent as the message's `osiris_version` attribute, and are
  written in the instance's pinned release. The events topic is in the spec as its `events` webhook,
  with the `EventDelivery` schema.
- Open strings: an event's `type`, an error's `code`, and GitHub's `account_type` and
  `repository_selection` may gain values without a new release; a client ignores one it doesn't
  know.
- Errors: a body or a query a route can't read is refused as `OS_02` (400) in the usual error
  shape, and a body larger than the server reads as `OS_10` (413). Every `/v1` operation documents 400, 401, 408, 413 and 500, and on a POST or DELETE 409
  and 422 for `Idempotency-Key`; actions and deleting an auth config document 404.
