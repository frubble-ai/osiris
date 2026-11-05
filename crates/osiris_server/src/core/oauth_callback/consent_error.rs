//! The reason saved when the provider sends the browser back with an OAuth `error`. The callback is
//! public and the reason reaches API clients as `last_error`, so only the error codes RFC 6749
//! defines are named; anything else gets a generic reason.

/// The error codes an authorization server may send back (RFC 6749, section 4.1.2.1).
const KNOWN: [&str; 7] = [
    "access_denied",
    "invalid_request",
    "unauthorized_client",
    "unsupported_response_type",
    "invalid_scope",
    "server_error",
    "temporarily_unavailable",
];

pub fn consent_error_reason(error: &str) -> String {
    match KNOWN.iter().find(|known| **known == error) {
        Some(known) => format!("the provider answered {known}"),
        None => "the provider refused the consent".to_owned(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn names_only_the_standard_error_codes() {
        assert_eq!(
            consent_error_reason("access_denied"),
            "the provider answered access_denied"
        );
        for sent in ["<script>", "access_denied_or_not", &"x".repeat(10_000)] {
            assert_eq!(
                consent_error_reason(sent),
                "the provider refused the consent"
            );
        }
    }
}
