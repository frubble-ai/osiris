//! Tests of `api_config`.

use super::*;
use osiris_common::secret::secret;

#[test]
fn refuses_an_admin_key_equal_to_the_api_key() {
    let key = secret("a-key-that-is-long-enough-0123456789");
    let error = ensure_distinct_keys(&key, &key.clone()).unwrap_err();
    assert!(error.to_string().contains("OSIRIS_ADMIN_KEY"));
    assert!(ensure_distinct_keys(&key, &secret("another-key-that-is-long-enough-01")).is_ok());
}
