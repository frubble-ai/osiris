# GitHub

GitHub, through our GitHub App: a user's OAuth consent, or the app acting as one of its
installations. The code is in `crates/osiris_providers/src/github/`, one folder per area of
GitHub's API and one file per action. OAuth endpoints, action names, inputs and scopes follow
`OpenConnector` v1.9.4 (Apache-2.0), `src/providers/github/`; the answers are osiris's own shapes.

## Actions

All 149 are listed in [actions](github/actions.md), generated from `scripts/actions/github/`. They
cover repositories (contents, branches, references, commits, collaborators, forks, topics),
issues, labels, milestones, comments and reactions, pull requests and reviews, statuses and check
runs, Actions workflows, runs, jobs, artifacts and logs, releases and their assets, activity
(events, stars, watching), search, users and installations. A list answers one page, as GitHub
pages it. A job's logs are answered as their last `[providers.listing] max_log_bytes`.

Optional text inputs are read as `OpenConnector` reads them: a blank one is left out and a padded
one sent without its surrounding whitespace. Titles, bodies, descriptions and commit messages are
sent exactly as given.

## Scopes

Each action's input declares the scopes it needs (`SCOPES`). GitHub's scopes nest, so a granted
scope covers the narrower ones (`repo` covers `public_repo` and `repo:status`). A token that
reports no scopes, like a GitHub App's user token, is not checked; GitHub refuses what the app may
not do.

## Installations

An auth config acts as the app's installations when it holds the app's credentials:

- `settings`: `{"app_id": "12345"}`, answered back.
- `secret_settings`: `{"private_key": "-----BEGIN …"}`, the PEM the app's settings page downloads;
  encrypted and never answered (`has_secret_settings` says it is set).

`POST /v1/connected-accounts` with `installation_id` connects as that installation, active at once.
Its tokens are minted with an RS256 App JWT when an action runs, kept in memory until
`[providers.installation_tokens] margin_seconds` before they expire (GitHub's last an hour), and
minted again once when GitHub refuses one.

An installation has no user, so on an installation connection:

- `get_current_user`, `list_installations`, `create_repository`, `star_repository`,
  `unstar_repository`, `check_repository_starred` and `list_my_starred_repositories` are refused
  with `OS_02` (400) before GitHub is called; they act as a user.
- `list_my_repositories` and `list_repositories` answer the installation's repositories
  (`GET /installation/repositories`). `list_my_repositories` refuses `visibility`, `sort` and
  `direction` there, which only a user's list takes.

## Triggers

| Kind | Config | Delivers |
| --- | --- | --- |
| `github.on_repo_event` | `{"owner", "repo", "events": ["push", …] or ["*"], "insecure_ssl"?}` | `event` (GitHub's event name), `delivery_id`, `payload` (GitHub's own) |
| `github.watch_pull_request` | `{"owner", "repo", "number"}` | `pull_request` and `version`, once per change |

`github.on_repo_event` creates a repository webhook that delivers to
`{public_url}/webhooks/triggers/{id}`, signed with the trigger's own secret (HMAC-SHA256,
`X-Hub-Signature-256`). If GitHub already has a webhook for that URL, it is updated instead.
Deleting the trigger deletes the webhook; one already gone counts as deleted. The events it takes
are `OpenConnector`'s list (`github/triggers/repo_event_config.rs`).

`github.watch_pull_request` also creates a webhook for the pull request's events, and checks the
pull request when one arrives and every `[triggers] check_seconds`. The first check records where
it starts; each later one publishes only a change it hasn't seen.

## Open strings

A repository's installation `account_type` and `repository_selection` may gain values without a
new release; a client ignores one it doesn't know.

## Live tests

`crates/osiris_providers/tests/live/github.rs` runs against a real account with `OSIRIS_LIVE_GITHUB_TOKEN`, nightly
(`live.yml`); it is skipped locally without the token and fails in CI.
