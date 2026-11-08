//! GitHub's action routes: pull requests.

pub mod check_pull_request_merged;
pub mod create_pull_request;
pub mod create_pull_request_review;
pub mod create_pull_request_review_comment;
pub mod delete_pending_pull_request_review;
pub mod delete_pull_request_review_comment;
pub mod dismiss_pull_request_review;
pub mod get_pull_request;
pub mod get_pull_request_review;
pub mod list_pull_request_commits;
pub mod list_pull_request_files;
pub mod list_pull_request_requested_reviewers;
pub mod list_pull_request_review_comments;
pub mod list_pull_request_reviews;
pub mod list_pull_requests;
pub mod list_pull_requests_associated_with_commit;
pub mod merge_pull_request;
pub mod remove_pull_request_reviewers;
pub mod reply_pull_request_review_comment;
pub mod request_pull_request_reviewers;
pub mod router;
pub mod submit_pull_request_review;
pub mod update_pull_request;
pub mod update_pull_request_branch;
pub mod update_pull_request_review_comment;

pub use router::router;
