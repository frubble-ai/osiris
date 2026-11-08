//! What the server's tests share: a configuration fixture, and a test context with the real use
//! cases, the API and worker routers, the test's own database, the providers' faked network (every
//! registered provider, and a fake one for the tests they share), an
//! in-memory event bus and an in-memory task scheduler. Tests arrange state through the use cases
//! and the routes, never by writing rows.

#![allow(
    clippy::missing_panics_doc,
    clippy::panic,
    clippy::expect_used,
    clippy::unwrap_used,
    reason = "test setup: a failure here is the test failing"
)]

pub mod arrange;
pub mod conformance;
pub mod db_fixtures;
pub mod generate_connect_attempt;
pub mod github;
pub mod golden;
pub mod linear;
pub mod mock_releases;
pub mod observe;
pub mod requests;
pub mod run_tasks;
pub mod slack;
pub mod test_config;
pub mod test_context;

pub use test_context::{
    ADMIN_KEY, API_KEY, TestContext, WORKER_TOKEN, setup_test_context, setup_test_context_with,
};
