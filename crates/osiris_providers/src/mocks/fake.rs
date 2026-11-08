//! A provider that exists only in tests: plain `OAuth2` with refresh tokens, an RFC 7009-style
//! revoke and one action. The shared tests (connecting, refreshing, disconnecting, the action
//! runner) run against it, so they need no real provider. It is never registered by `build.rs`;
//! a test adds [`FAKE_REGISTRATION`] itself.

pub mod fake_provider;
pub mod mock_fake_network;
pub mod registration;
pub mod revoke;
pub mod whoami;

pub use fake_provider::{FAKE_ID, FakeProvider};
pub use mock_fake_network::MockFakeNetwork;
pub use registration::FAKE_REGISTRATION;
