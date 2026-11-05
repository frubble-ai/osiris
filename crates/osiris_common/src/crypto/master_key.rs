//! The master key: derived (HKDF-SHA256) from the `OSIRIS_ENCRYPTION_KEY` secret, so the secret can
//! be any long random string. It encrypts nothing but data keys.

use super::{Cipher, CryptoError};
use hkdf::Hkdf;
use sha2::Sha256;

#[derive(Clone)]
pub struct MasterKey {
    cipher: Cipher,
}

impl MasterKey {
    pub fn from_secret(secret: &str) -> Result<Self, CryptoError> {
        if secret.len() < 32 {
            return Err(CryptoError::ShortKey);
        }
        let mut key = [0u8; 32];
        Hkdf::<Sha256>::new(None, secret.as_bytes())
            .expand(b"osiris master key v1", &mut key)
            .map_err(|_| CryptoError::ShortKey)?;
        Ok(Self {
            cipher: Cipher::new(&key),
        })
    }

    /// A fresh random data key: its cipher, and the key wrapped by this master key, to store.
    pub fn new_data_key(&self) -> Result<(Cipher, Vec<u8>), CryptoError> {
        let mut key = [0u8; 32];
        rand::fill(&mut key);
        Ok((Cipher::new(&key), self.cipher.encrypt_bytes(&key)?))
    }

    /// The cipher of a stored, wrapped data key.
    pub fn unwrap_data_key(&self, wrapped: &[u8]) -> Result<Cipher, CryptoError> {
        let key: [u8; 32] = self
            .cipher
            .decrypt_bytes(wrapped)?
            .try_into()
            .map_err(|_| CryptoError::Decrypt)?;
        Ok(Cipher::new(&key))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const SECRET: &str = "a-test-secret-that-is-long-enough-for-osiris";

    #[test]
    fn unwraps_the_data_key_it_wrapped() {
        let master = MasterKey::from_secret(SECRET).unwrap();
        let (cipher, wrapped) = master.new_data_key().unwrap();
        let stored = cipher.encrypt("a-token").unwrap();
        assert_eq!(
            master
                .unwrap_data_key(&wrapped)
                .unwrap()
                .decrypt(&stored)
                .unwrap(),
            "a-token"
        );
    }

    #[test]
    fn refuses_a_data_key_another_master_wrapped_and_a_short_secret() {
        let (_, wrapped) = MasterKey::from_secret(SECRET)
            .unwrap()
            .new_data_key()
            .unwrap();
        let other = MasterKey::from_secret("another-secret-that-is-also-long-enough").unwrap();
        assert!(other.unwrap_data_key(&wrapped).is_err());
        assert!(MasterKey::from_secret("short").is_err());
    }
}
