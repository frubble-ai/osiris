//! Tests of `process`.

use super::*;
use rstest::rstest;

#[test]
fn selects_each_process() {
    assert_eq!(Process::parse(Some("serve")).unwrap(), Process::Serve);
    assert_eq!(Process::parse(Some("worker")).unwrap(), Process::Worker);
    assert_eq!(Process::parse(Some("migrate")).unwrap(), Process::Migrate);
}

#[test]
fn refuses_a_missing_process() {
    let missing = Process::parse(None).unwrap_err().to_string();
    assert!(missing.contains("OSIRIS_PROCESS is required"), "{missing}");
}

#[rstest]
#[case::empty("")]
#[case::api("api")]
#[case::scheduler("scheduler")]
#[case::wrong_case("Serve")]
fn refuses_an_unknown_process(#[case] value: &str) {
    let error = Process::parse(Some(value)).unwrap_err().to_string();
    assert!(error.contains("OSIRIS_PROCESS"), "{error}");
}
