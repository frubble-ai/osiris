//! Tests of `default_callback_url`.

use super::*;
use rstest::rstest;

#[rstest]
#[case::local("http://localhost:8090", "http://localhost:8090/health")]
#[case::remote("https://osiris.example", "https://osiris.example/health")]
#[case::trailing_slash("https://osiris.example/", "https://osiris.example/health")]
fn points_at_the_osiris_of_the_profile(#[case] profile_url: &str, #[case] expected: &str) {
    assert_eq!(default_callback_url(profile_url), expected);
}
