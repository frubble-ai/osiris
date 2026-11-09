//! A value placed in a URL path, percent-encoded so an owner, a branch or a file name can't add a
//! segment to the path it is placed in. A value of `.` or `..` stays a segment the URL parser
//! would collapse, so every request's path is checked before it is sent
//! ([`request_path`](crate::request_path)), which refuses one.

use std::fmt::Write;

/// `value` with every byte but the unreserved ones (letters, digits, `-`, `.`, `_`, `~`)
/// percent-encoded.
pub fn path_segment(value: &str) -> String {
    let mut encoded = String::with_capacity(value.len());
    for byte in value.bytes() {
        if byte.is_ascii_alphanumeric() || matches!(byte, b'-' | b'.' | b'_' | b'~') {
            encoded.push(char::from(byte));
        } else {
            let _ = write!(encoded, "%{byte:02X}");
        }
    }
    encoded
}

#[cfg(test)]
mod tests;
