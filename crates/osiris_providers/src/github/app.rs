//! Acting as our GitHub App's installation: the app's credentials (its id and private key, kept
//! in the auth config's settings), the short JWT that identifies the app, and the installation
//! token minted with it. As in `OpenConnector` v1.9.4 (Apache-2.0),
//! `src/providers/github/app-installation.ts`.

pub mod app_credentials;
pub mod app_jwt;
pub mod mint_installation_token;

pub use app_credentials::AppCredentials;

#[cfg(test)]
mod tests;
