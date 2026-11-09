//! GitHub's commit statuses and check runs faked at the network, for tests.

pub mod generate_github_check_run;
pub mod generate_github_commit_status;
pub mod mock_github_checks_network;

pub use mock_github_checks_network::MockGithubChecksNetwork;
