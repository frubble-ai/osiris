//! An optional text input as `OpenConnector` reads it before sending it: surrounding whitespace
//! removed, and left out when nothing remains.

/// `value` without its surrounding whitespace, or `None` when it is absent or blank.
pub fn trimmed(value: Option<&str>) -> Option<&str> {
    value.map(str::trim).filter(|value| !value.is_empty())
}
