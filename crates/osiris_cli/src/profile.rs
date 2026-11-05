//! The CLI's profile: where osiris is and the keys to call it with, written by `login` and readable
//! by its owner only. One operation per module.

pub mod load;
pub mod load_if_present;
pub mod path;
pub mod save;
pub mod types;

pub use load::load;
pub use load_if_present::load_if_present;
pub use save::save;
pub use types::Profile;

#[cfg(test)]
mod tests;
