//! Slack's Web API faked at the network, for tests: the real provider code runs, only the HTTP
//! answers are the test's.

pub mod generate_slack_channel;
pub mod mock_slack_network;

pub use mock_slack_network::MockSlackNetwork;
