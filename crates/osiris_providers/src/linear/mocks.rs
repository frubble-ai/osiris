//! Linear's API faked at the network, for tests: the real provider code runs, only the HTTP answers
//! are the test's.

pub mod generate_linear_team;
pub mod mock_linear_network;

pub use mock_linear_network::MockLinearNetwork;
