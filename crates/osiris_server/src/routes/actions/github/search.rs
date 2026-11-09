//! GitHub's action routes: search.

pub mod router;
pub mod search_code;
pub mod search_commits;
pub mod search_labels;
pub mod search_repositories;
pub mod search_topics;
pub mod search_users;

pub use router::router;
