//! A commit's SHA: 40 hex characters.

use fake::Fake;

/// A random 40-character hex SHA.
pub fn generate_sha() -> String {
    format!(
        "{:016x}{:016x}{:08x}",
        (..u64::MAX).fake::<u64>(),
        (..u64::MAX).fake::<u64>(),
        (..u32::MAX).fake::<u32>()
    )
}
