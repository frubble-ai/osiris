//! Where the browser goes after a consent when `--callback-url` is left out: the osiris the
//! profile points at, so a remote profile does not send the browser to localhost.

pub fn default_callback_url(profile_url: &str) -> String {
    format!("{}/health", profile_url.trim_end_matches('/'))
}

#[cfg(test)]
mod tests;
