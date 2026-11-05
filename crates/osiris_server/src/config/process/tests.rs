//! Tests of `process`.

use super::*;

#[test]
fn selects_each_process() {
    assert_eq!(Process::parse(Some("serve")).unwrap(), Process::Serve);
    assert_eq!(Process::parse(Some("worker")).unwrap(), Process::Worker);
    assert_eq!(Process::parse(Some("migrate")).unwrap(), Process::Migrate);
}

#[test]
fn refuses_a_missing_or_unknown_process() {
    let missing = Process::parse(None).unwrap_err().to_string();
    assert!(missing.contains("OSIRIS_PROCESS is required"), "{missing}");
    for value in ["", "api", "scheduler", "Serve"] {
        let error = Process::parse(Some(value)).unwrap_err().to_string();
        assert!(error.contains("OSIRIS_PROCESS"), "{value}: {error}");
    }
}
