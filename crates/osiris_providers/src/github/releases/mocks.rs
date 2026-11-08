//! GitHub's releases faked for tests: the releases and assets it answers, and the requests that
//! carry a body.

pub mod generate_github_release;
pub mod generate_github_release_asset;
pub mod mock_github_release_requests;

pub use mock_github_release_requests::MockGithubReleaseRequests;
