//! Search across GitHub: repositories, issues, users, commits, code, labels and topics.

#[cfg(any(test, feature = "testing"))]
pub mod mocks;
pub mod search_code;
pub mod search_commits;
pub mod search_labels;
pub mod search_repositories;
pub mod search_topics;
pub mod search_users;

pub use search_code::SearchCode;
pub use search_commits::SearchCommits;
pub use search_labels::SearchLabels;
pub use search_repositories::SearchRepositories;
pub use search_topics::SearchTopics;
pub use search_users::SearchUsers;

#[cfg(test)]
mod tests;
