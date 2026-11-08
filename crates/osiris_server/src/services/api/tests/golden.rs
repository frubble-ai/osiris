//! Each release's golden transcripts, replayed: every scenario runs at every release osiris
//! serves, on a database of its own, and must still do what was recorded for that release
//! (`testing::golden`). `for_every_release!` lists the releases, `golden!` makes a scenario's test
//! at each, and `tests` fails until every release osiris serves is listed. The shared scenarios run
//! against the fake provider; each provider's own are in its module.

#[macro_use]
mod releases;
#[macro_use]
mod golden_at;

mod auth_configs;
mod connecting;
mod events;
mod github;
mod idempotency;
mod refusals;
mod slack;
mod tests;
