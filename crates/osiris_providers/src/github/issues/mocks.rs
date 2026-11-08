//! GitHub's issue answers, generated for tests, and the arrangers that check what an issue action
//! sends.

pub mod generate_github_issue;
pub mod generate_github_issue_comment;
pub mod generate_github_issue_event;
pub mod generate_github_label;
pub mod generate_github_milestone;
pub mod generate_github_reaction;
pub mod generate_github_search_issue;
pub mod generate_github_timeline_event;
pub mod mock_github_issues_network;

pub use mock_github_issues_network::MockGithubIssuesNetwork;
