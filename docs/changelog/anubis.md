# 2026-11-04.anubis

The first release, a **preview**: its contract may still change until it is cut as stable
(`Osiris-Stability` says which).

- Auth configs: register, list, read, update and delete providers' apps (admin key).
- Connected accounts: open a consent, list (by user, provider, auth config, alias, status), read,
  delete (the grant is ended at the provider), and refresh now (admin key). Each connection reports
  `setup_complete`. A connection to a provider that issues refresh tokens is only `active` when its
  tokens and their scheduled refresh are both saved; otherwise it is `disabled`. `refresh_task` and
  `refresh_task_at` show the pending refresh.
- Actions: `linear.list_teams`, `slack.list_channels`, and 149 GitHub actions, with the same
  functions as OpenConnector v1.9.4's GitHub provider but answering in osiris's own shapes:
  repositories (contents, branches, commits, collaborators, forks, topics), issues, pull requests
  and reviews, releases and their assets, Actions workflows, runs, jobs and logs, check runs,
  activity (notifications, stars, watching), search, users and installations. Each runs at
  `POST /v1/connected-accounts/{id}/actions/github.<name>`, and a list answers in osiris's
  pages.
- Scopes: an action whose connection's token lacks the scopes it needs is refused before GitHub is
  called, as `OS_11` (403), with the missing scopes named. GitHub's nested scopes count (`repo`
  covers `public_repo`). A token that reports no scopes, such as a GitHub App's, is not checked.
- GitHub App installations: an auth config can hold the app's `settings` (`app_id`) and
  `secret_settings` (`private_key`, encrypted and never returned; `has_secret_settings` says
  whether it is set). `POST /v1/connected-accounts` with `installation_id` connects as that
  installation, with no consent, and is `active` at once. Its tokens are minted when an action runs,
  reused until `[providers.installation_tokens] margin_seconds` before they expire, and minted once
  more when GitHub refuses one. A connection reports `credential_kind` (`oauth` or
  `app_installation`) and `installation_id`.
- Triggers: `/v1/triggers` creates, lists, reads and deletes a connection's triggers.
  `github.on_repo_event` creates a repository webhook for the chosen events (or `*`).
  `github.watch_pull_request` watches one pull request, checked on its own deliveries and every
  `[triggers] check_seconds`. Each delivery or change is published as a `trigger.fired` event
  carrying `trigger_id`, `trigger` and `data`. Deleting a trigger removes its webhook. A trigger
  the provider refuses for good turns `failed`.
- Events: `connection.active`, `connection.expired`, `connection.revoked`, `trigger.fired`; test events with
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
