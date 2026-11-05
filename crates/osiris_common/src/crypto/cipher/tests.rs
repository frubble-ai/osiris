//! Tests of `cipher`.

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
