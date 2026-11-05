//! A secret in and out of its stored form.

use crate::errors::{StorageError, StorageResult};
use error_stack::ResultExt;
use osiris_common::crypto::Cipher;

pub fn decrypt(cipher: &Cipher, value: &[u8]) -> StorageResult<String> {
    cipher.decrypt(value).change_context(StorageError::Decrypt)
}

pub fn encrypt(cipher: &Cipher, value: &str) -> StorageResult<Vec<u8>> {
    cipher.encrypt(value).change_context(StorageError::Encrypt)
}
