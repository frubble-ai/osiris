//! GitHub's users: the authenticated one.

pub mod get_current_user;

pub use get_current_user::GetCurrentUser;

#[cfg(test)]
mod tests;
