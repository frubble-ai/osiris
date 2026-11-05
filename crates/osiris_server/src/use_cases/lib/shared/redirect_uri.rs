//! Where providers send the browser back after a consent.

pub fn redirect_uri(public_url: &str) -> String {
    format!("{public_url}/oauth/callback")
}
