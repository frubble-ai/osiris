//! What every provider's fake answers with (a token answer), and a provider that exists only in
//! tests, for the tests every provider shares.

pub mod generate_token_answer;
pub mod mock_fake_network;
pub mod mock_fake_provider;

pub use mock_fake_network::MockFakeNetwork;
pub use mock_fake_provider::{FAKE_ID, FAKE_REGISTRATION, FakeProvider};
