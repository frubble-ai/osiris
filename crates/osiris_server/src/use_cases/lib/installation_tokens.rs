//! App installations' tokens, minted on demand and reused until shortly before they expire. Only
//! this process holds them, in memory: they are never stored, and a restart mints new ones.

pub mod connection_token;
pub mod installation_token;
pub mod installation_token_cache;
pub mod mint_failed;

pub use installation_token_cache::InstallationTokenCache;

#[cfg(test)]
mod tests;
