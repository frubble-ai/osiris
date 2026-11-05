//! A secret in and out of its stored form.

use crate::use_cases::UseCaseResult;
use error_stack::ResultExt;
use osiris_common::crypto::Cipher;
use osiris_domain::DomainError;

pub fn decrypt(cipher: &Cipher, value: &[u8]) -> UseCaseResult<String> {
    cipher.decrypt(value).change_context(DomainError::Internal)
}

pub fn encrypt(cipher: &Cipher, value: &str) -> UseCaseResult<Vec<u8>> {
    cipher.encrypt(value).change_context(DomainError::Internal)
}
