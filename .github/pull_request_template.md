<!--
PR title = squash commit message. Use Conventional Commits: feat|fix|chore|docs|refactor|ci(scope): summary
-->

## Why

<!-- The problem this solves. Link the ticket so it closes on merge: Closes CUBE-123 -->

## What changed

<!-- Short summary. The diff shows the details; tell reviewers what to focus on. -->

## How I tested it

<!-- Steps a reviewer can repeat, and what you saw. Paste command output or URLs. -->

## Risk and rollback

<!-- What could break and who notices. How to undo it.
For deploys: a migration, new settings, env vars or secrets that must exist first? An API change
that needs a new version (docs/api-versioning.md)? -->

## Checklist

- [ ] `just check` passes: format, clippy, cargo-deny, `.sqlx/`, the tests, the OpenAPI spec
- [ ] A migration added is reversible, and ran, reverted and ran again
- [ ] New settings are in `config.example.toml`; new env vars or secrets are listed above
- [ ] No token, client secret or key in code, logs, errors or screenshots
- [ ] Docs updated if behaviour, the API or a manual step changed
