//! The `OAuth2` authorization-code flow for any provider, driven by its [`OAuth2Definition`].

pub mod authorize;
pub mod definition;
pub mod pkce;
pub mod token;

pub use authorize::authorize_url;
pub use definition::{ClientAuth, OAuth2Definition};
pub use pkce::pkce_verifier;
pub use token::{exchange_code, refresh};
