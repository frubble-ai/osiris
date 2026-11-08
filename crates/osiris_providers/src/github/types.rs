//! GitHub's resources in our terms, one file each: the shapes osiris's GitHub actions answer with,
//! the fields `OpenConnector`'s GitHub output schemas name (v1.9.4, Apache-2.0), typed, with
//! GitHub's numeric ids as strings.

pub mod branch;
pub mod installation;
pub mod repository;
pub mod user;

pub use branch::GithubBranch;
pub use installation::GithubInstallation;
pub use repository::GithubRepository;
pub use user::{GithubCurrentUser, GithubUser};
