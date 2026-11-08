//! The masker hides only what changes from run to run: an id keeps one placeholder across a
//! transcript, times, long random tokens and the faked providers' origin are replaced, and every
//! other word is left as it is, so a transcript still shows what osiris answered.

use super::Masker;
use rstest::rstest;
use serde_json::json;

#[test]
fn gives_the_same_id_the_same_placeholder_and_each_prefix_its_own_count() {
    let mut masker = Masker::default();
    let first = "ac_0123456789abcdefABCD";
    let second = "ac_ZYXWVUTSRQ9876543210";
    let connection = "ca_0123456789abcdefABCD";

    let placeholders = masker.mask(&json!([
        first,
        second,
        first,
        connection,
        format!("/v1/x/{first}")
    ]));

    assert_eq!(
        placeholders,
        json!(["<ac_1>", "<ac_2>", "<ac_1>", "<ca_1>", "/v1/x/<ac_1>"])
    );
}

#[rstest]
#[case::a_time("2026-11-04T07:39:16.395656Z", "<time>")]
#[case::a_long_token("state=AbCdEfGhIjKlMnOpQrStUvWxYz", "state=<random>")]
#[case::the_provider_s_origin("http://127.0.0.1:61234/login", "<provider>/login")]
fn replaces_what_changes_from_run_to_run(#[case] text: &str, #[case] masked: &str) {
    assert_eq!(
        Masker::new("http://127.0.0.1:61234/").mask_text(text),
        masked
    );
}

#[rstest]
#[case::a_word("connection.active")]
#[case::a_code("OS_02")]
#[case::a_short_prefixed_word("ac_short")]
#[case::an_unprefixed_word("golden-github-app")]
#[case::a_number("28800")]
#[case::a_word_with_an_upper_case_prefix("AC_0123456789abcdefABCD")]
fn leaves_every_other_word_as_it_is(#[case] text: &str) {
    assert_eq!(Masker::default().mask_text(text), text);
}
