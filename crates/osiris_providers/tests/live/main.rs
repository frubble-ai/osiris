//! Against the real providers, with test accounts' tokens: what the fakes can't tell us (an API
//! that changed its shape, a scope that stopped being enough). `#[ignore]`: run nightly, or with
//! `cargo nextest run -p osiris_providers --run-ignored only`. Locally, a provider whose token is
//! unset is skipped with a note; in CI (`CI` set) a missing token fails the test, so the nightly run
//! can't pass having checked nothing. Each provider's checks are in its own module.

#![allow(
    clippy::unwrap_used,
    clippy::print_stderr,
    reason = "test fixtures: a failure here is the test failing; a skip is reported on stderr"
)]

mod github;
mod limits;
mod linear;
mod slack;
mod token;
mod urls;
