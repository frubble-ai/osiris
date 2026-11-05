# Style Guide

How code in osiris is written. The rules match frubble-ai/app's style guide where they apply,
and Hyperswitch's conventions for a Rust service.

| Guide | What it covers |
| --- | --- |
| [Placement](./placement.md) | domains in every layer, one unit per file, `foo.rs` + `foo/` |
| [Settings](./settings.md) | tuning values in layered settings, never constants or SQL literals |
| [Errors](./errors.md) | an error type per layer, `ErrorSwitch`, error-stack reports, `OS_xx` on the wire |
| [Testing](./testing.md) | real Postgres per test, fakes for providers, what a test asserts |
| [Migrations](./migrations.md) | reversible SQL, never at startup, checked queries |

Also: Hyperswitch's workspace lints are on as warnings and CI fails on any; `clippy.toml` lets
tests unwrap. Names are words, not letters. Comments say why, not what.
