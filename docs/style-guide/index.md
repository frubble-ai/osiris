# Style Guide

How code in osiris is written. The rules mirror frubble-ai/app's backend, translated to Rust: the same use-case pattern, context, configuration model, thin callers, placement rule and test organization.

| Guide | What it covers |
| --- | --- |
| [Use cases](./use-cases.md) | the context, `UseCases`, one case per file, `lib/` on parts of the context, thin callers |
| [Placement rule](./placement-rule.md) | where a unit of code lives; domains in every layer; `foo.rs` + `foo/` |
| [Third-party integrations](./third-party-integrations.md) | a crate per vendor behind a trait of ours; contract tests |
| [Test organization](./test-organization.md) | tests, mocks and factories beside the code; `testing/`; no inline mocks |
| [Testing patterns](./testing-patterns.md) | what is tested where, and what a test asserts |
| [Settings](./settings.md) | configuration from the environment, tuning values in settings |
| [Errors](./errors.md) | an error type per layer, `ErrorSwitch`, error-stack reports, `OS_xx` on the wire |
| [Migrations](./migrations.md) | reversible SQL, never at startup, checked queries |

Also: Hyperswitch's workspace lints are on as warnings and CI fails on any; `clippy.toml` lets tests unwrap. Names are words, not letters. A comment says what a unit does and why, in complete sentences.
