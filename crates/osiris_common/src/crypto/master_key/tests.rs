//! Tests of `master_key`.

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
