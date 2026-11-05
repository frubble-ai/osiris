//! A setting's seconds as a duration.

pub(crate) fn seconds(value: u64) -> time::Duration {
    time::Duration::seconds(i64::try_from(value).unwrap_or(i64::MAX))
}
