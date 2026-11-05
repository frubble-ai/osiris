//! GitHub's API faked at the network, for tests: the real provider code runs, only the HTTP answers
//! are the test's.

pub mod generate_github_installation;
pub mod generate_github_repository;
pub mod mock_github_network;

pub use mock_github_network::MockGithubNetwork;
