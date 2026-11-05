# 2026-11-04.anubis

The first release.

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
