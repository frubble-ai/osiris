# 2026-11-04.anubis

The first release, a **preview**: its contract may still change until it is cut as stable
(`Osiris-Stability` says which).

- Auth configs: register, list, read, update and delete providers' apps (admin key).
- Connected accounts: open a consent, list (by user, provider, auth config, alias, status), read,
  delete (the grant is ended at the provider), and refresh now (admin key). Each connection reports
  `setup_complete`. A connection to a provider that issues refresh tokens is only `active` when its
  tokens and their scheduled refresh are both saved; otherwise it is `disabled`. `refresh_task` and
  `refresh_task_at` show the pending refresh.
- Actions: one route per action, `POST /v1/connected-accounts/{id}/actions/<provider>.<action>`,
  each with its own input and answer in the spec; a list answers in osiris's pages. Each
  provider's actions are listed on its page in [providers](../providers.md), with the same
  functions as OpenConnector v1.9.4's but answering in osiris's own shapes.
- Scopes: an action whose connection's token lacks the scopes it needs is refused before the
  provider is called, as `OS_11` (403), with the missing scopes named. A provider whose scopes
  nest counts a wider one as covering the narrower ones. A token that reports no scopes is not
  checked.
- App installations: an auth config can hold its app's `settings` and `secret_settings` (encrypted
  and never returned; `has_secret_settings` says whether they are set), which the provider
  validates. `POST /v1/connected-accounts` with `installation_id` (admin key) connects as that installation,
  with no consent, and is `active` at once. Its tokens are minted when an action runs, reused
  until `[providers.installation_tokens] margin_seconds` before they expire, and minted once more
  when the provider refuses one. An action that only works as a user is refused on one as
  `OS_02` (400) before the provider is called. A connection reports `credential_kind` (`oauth` or
  `app_installation`) and `installation_id`.
- Triggers: `/v1/triggers` creates, lists, reads and deletes a connection's triggers, of the kinds
  its provider offers (listed on the provider's page). A provider's webhook delivers to
  `/webhooks/triggers/{id}`, signed with the trigger's own secret; a kind that also checks on a
  schedule does so every `[triggers] check_seconds`. Each delivery or change is published as a
  `trigger.fired` event carrying `trigger_id`, `trigger` and `data`. Deleting a trigger removes its
  subscription at the provider. A trigger the provider refuses for good turns `failed`.
- Events: `connection.active`, `connection.expired`, `connection.revoked`, `trigger.fired`; test events with
  `POST /v1/events/trigger` (admin key).
- Versions: every answer names its release in `Osiris-Version` and its stability in
  `Osiris-Stability`. An answer at a superseded release carries `Deprecation` and `Sunset`.
- Events carry `osiris_version`, also sent as the message's `osiris_version` attribute, and are
  written in the instance's pinned release. The events topic is in the spec as its `events` webhook,
  with the `EventDelivery` schema.
- Open strings: an event's `type`, an error's `code`, and the open values each provider's page
  lists may gain values without a new release; a client ignores one it doesn't know.
- Errors: a body or a query a route can't read is refused as `OS_02` (400) in the usual error
  shape, and a body larger than the server reads as `OS_10` (413). Every `/v1` operation documents 400, 401, 408, 413 and 500, and on a POST or DELETE 409
  and 422 for `Idempotency-Key`; actions and deleting an auth config document 404.
