//! The test account's token, read from the environment.

/// The token in `name`; none when it is unset, which fails the test in CI.
pub fn token(name: &str) -> Option<String> {
    let token = std::env::var(name).ok().filter(|value| !value.is_empty());
    if token.is_none() {
        assert!(std::env::var_os("CI").is_none(), "{name} is not set in CI");
        eprintln!("skipped: {name} is not set");
    }
    token
}
