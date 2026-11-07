//! The digest a release is locked to: the SHA-256 of its contract, as compact JSON with its keys
//! in order.

use super::contract;
use serde_json::Value;
use sha2::{Digest, Sha256};
use std::fmt::Write;

pub fn contract_sha256(spec: &Value) -> String {
    // serde_json's maps keep their keys in order, so the same contract always gives the same bytes.
    let bytes = serde_json::to_vec(&contract(spec)).unwrap_or_default();
    Sha256::digest(&bytes)
        .iter()
        .fold(String::with_capacity(64), |mut hex, byte| {
            let _ = write!(hex, "{byte:02x}");
            hex
        })
}
