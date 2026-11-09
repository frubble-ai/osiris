# Slack

Slack: a user token through Slack's user-token flow, so the auth config's scopes are the user's.
The code is in `crates/osiris_providers/src/slack/`. Endpoints, action names and scopes follow
`OpenConnector` v1.9.4 (Apache-2.0), `src/providers/slack/`.

- Actions: listed in [actions](slack/actions.md), generated from `scripts/actions/slack/`.
  `slack.list_channels` needs `channels:read`.
- Slack's Web API answers `ok: false` beside a 200; `slack/transformers.rs` reads it by its error
  code (a revoked token, a missing scope, a rate limit or an outage).
- Its methods join onto `https://slack.com/api/`, so its API URL ends with `/`.
- Refresh tokens come when the app has token rotation turned on.

## Live tests

`crates/osiris_providers/tests/live/slack.rs` runs against a real account with `OSIRIS_LIVE_SLACK_TOKEN`, nightly
(`live.yml`); it is skipped locally without the token and fails in CI.
