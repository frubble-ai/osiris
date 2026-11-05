//! Encryption at rest, as envelopes: a master key (from `OSIRIS_ENCRYPTION_KEY`) wraps one random
//! data key per tenant, and each tenant's tokens and secrets are encrypted with its own data key.
//! Every value is AES-256-GCM with a fresh nonce: `version (1 byte) | nonce (12) | ciphertext`.

pub mod cipher;
pub mod error;
pub mod master_key;

pub use cipher::Cipher;
pub use error::CryptoError;
pub use master_key::MasterKey;
