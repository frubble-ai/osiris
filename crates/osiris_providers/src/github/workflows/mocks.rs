//! GitHub Actions faked at the network, for tests.

pub mod generate_github_artifact;
pub mod generate_github_workflow;
pub mod generate_github_workflow_job;
pub mod generate_github_workflow_run;
pub mod mock_github_workflows_network;

pub use mock_github_workflows_network::MockGithubWorkflowsNetwork;
