//! GitHub's users: the authenticated one, and anyone's public profile.

pub mod get_current_user;
pub mod get_user;

pub use get_current_user::GetCurrentUser;
pub use get_user::GetUser;

#[cfg(any(test, feature = "testing"))]
pub mod mocks;
#[cfg(test)]
mod tests;
