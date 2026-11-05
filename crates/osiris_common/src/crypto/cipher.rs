//! AES-256-GCM over one 32-byte key: a tenant's data key, or the master key wrapping them.

use super::CryptoError;
use aes_gcm::{
    Aes256Gcm, Nonce,
    aead::{Aead, KeyInit},
};

/// The stored format's version, the first byte of every value.
const VERSION: u8 = 1;
const NONCE_LEN: usize = 12;

#[derive(Clone)]
pub struct Cipher {
    aead: Aes256Gcm,
}

impl Cipher {
    pub fn new(key: &[u8; 32]) -> Self {
        Self {
            aead: Aes256Gcm::new(key.into()),
        }
    }

    pub fn encrypt_bytes(&self, plaintext: &[u8]) -> Result<Vec<u8>, CryptoError> {
        let mut nonce = [0u8; NONCE_LEN];
        rand::fill(&mut nonce);
        let ciphertext = self
            .aead
            .encrypt(&Nonce::from(nonce), plaintext)
            .map_err(|_| CryptoError::Encrypt)?;
        let mut stored = Vec::with_capacity(1 + NONCE_LEN + ciphertext.len());
        stored.push(VERSION);
        stored.extend_from_slice(&nonce);
        stored.extend_from_slice(&ciphertext);
        Ok(stored)
    }

    pub fn decrypt_bytes(&self, stored: &[u8]) -> Result<Vec<u8>, CryptoError> {
        let (version, rest) = stored.split_first().ok_or(CryptoError::Decrypt)?;
        if *version != VERSION {
            return Err(CryptoError::Decrypt);
        }
        let (nonce, ciphertext) = rest
            .split_at_checked(NONCE_LEN)
            .ok_or(CryptoError::Decrypt)?;
        let nonce: [u8; NONCE_LEN] = nonce.try_into().map_err(|_| CryptoError::Decrypt)?;
        self.aead
            .decrypt(&Nonce::from(nonce), ciphertext)
            .map_err(|_| CryptoError::Decrypt)
    }

    pub fn encrypt(&self, plaintext: &str) -> Result<Vec<u8>, CryptoError> {
        self.encrypt_bytes(plaintext.as_bytes())
    }

    pub fn decrypt(&self, stored: &[u8]) -> Result<String, CryptoError> {
        String::from_utf8(self.decrypt_bytes(stored)?).map_err(|_| CryptoError::Decrypt)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn cipher() -> Cipher {
        Cipher::new(&[7u8; 32])
    }

    #[test]
    fn decrypts_what_it_encrypted() {
        let stored = cipher().encrypt("a-token").unwrap();
        assert_eq!(cipher().decrypt(&stored).unwrap(), "a-token");
    }

    #[test]
    fn never_stores_the_same_bytes_twice() {
        assert_ne!(
            cipher().encrypt("same").unwrap(),
            cipher().encrypt("same").unwrap()
        );
    }

    #[test]
    fn refuses_a_value_another_key_encrypted_or_one_tampered_with() {
        let stored = cipher().encrypt("a-token").unwrap();
        assert!(Cipher::new(&[8u8; 32]).decrypt(&stored).is_err());
        let mut tampered = stored.clone();
        if let Some(last) = tampered.last_mut() {
            *last ^= 1;
        }
        assert!(cipher().decrypt(&tampered).is_err());
        assert!(cipher().decrypt(&[]).is_err());
        assert!(cipher().decrypt(&[1, 2, 3]).is_err());
    }
}
