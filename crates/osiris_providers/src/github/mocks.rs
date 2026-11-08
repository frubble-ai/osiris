//! GitHub's API faked at the network, for tests: the real provider code runs, only the HTTP answers
//! are the test's.

pub mod generate_github_branch;
pub mod generate_github_installation;
pub mod generate_github_repository;
pub mod generate_github_user;
pub mod generate_sha;
pub mod github_on;
pub mod mock_github_app_key;
pub mod mock_github_network;
pub mod sign_github_delivery;

pub use mock_github_network::MockGithubNetwork;
