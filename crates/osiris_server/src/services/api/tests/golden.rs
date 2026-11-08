//! Each release's golden transcripts, replayed: every scenario runs at every release osiris
//! serves, on a database of its own, and must still do what was recorded for that release
//! (`testing::golden`). `golden_at!` makes a release's tests, `releases` lists them, and `tests`
//! fails until every release has every scenario's transcript.

#[macro_use]
mod golden_at;

mod actions;
mod auth_configs;
mod connecting;
mod events;
mod idempotency;
mod refusals;
mod releases;
mod scenarios;
mod tests;
