//! What encryption can fail with. Never says why a value didn't decrypt.

#[derive(Debug, thiserror::Error)]
pub enum CryptoError {
    #[error("the encryption key must be at least 32 characters")]
    ShortKey,
    #[error("a value could not be encrypted")]
    Encrypt,
    #[error("a stored value could not be decrypted")]
    Decrypt,
}
