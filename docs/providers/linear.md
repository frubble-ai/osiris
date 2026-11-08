# Linear

Linear: a user's OAuth consent, and reads through its GraphQL API. The code is in
`crates/osiris_providers/src/linear/`. Endpoints, action names and scopes follow `OpenConnector`
v1.9.4 (Apache-2.0), `src/providers/linear/`.

- Actions: listed in [actions](linear/actions.md), generated from `scripts/actions/linear/`.
  `linear.list_teams` needs `read`.
- Linear answers a GraphQL error beside a 200; `linear/graphql.rs` reads it, an authentication
  error as a rejected token.
- Every token comes with a refresh token, so connections are refreshed on schedule.

## Live tests

`crates/osiris_providers/tests/live/linear.rs` runs against a real account with `OSIRIS_LIVE_LINEAR_TOKEN`, nightly
(`live.yml`); it is skipped locally without the token and fails in CI.
