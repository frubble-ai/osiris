//! What a connection's token acts as, for an action whose call depends on it.

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ActingAs {
    /// The user who consented.
    User,
    /// An installation of the provider's app: no user, only what the installation was given.
    Installation,
}
