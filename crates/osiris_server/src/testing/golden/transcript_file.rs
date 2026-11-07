//! A transcript's file, `api-reference/golden/<release>/<scenario>.json`: written when recording,
//! otherwise read and held against what the scenario did now.

use super::contains::contains;
use serde_json::Value;
use std::path::PathBuf;

/// Where `scenario`'s transcript at `release` is kept.
pub fn transcript_path(release: &str, scenario: &str) -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("../../api-reference/golden")
        .join(release)
        .join(format!("{scenario}.json"))
}

/// Records `actual` when `OSIRIS_GOLDEN=record`; otherwise fails when it no longer holds the
/// recorded transcript.
pub fn check_transcript(release: &str, scenario: &str, actual: &Value) {
    let path = transcript_path(release, scenario);
    if std::env::var("OSIRIS_GOLDEN").as_deref() == Ok("record") {
        std::fs::create_dir_all(path.parent().expect("a directory")).expect("the golden directory");
        let text = serde_json::to_string_pretty(actual).expect("a transcript's JSON");
        std::fs::write(&path, format!("{text}\n")).expect("the transcript is written");
        return;
    }
    let Ok(text) = std::fs::read_to_string(&path) else {
        panic!(
            "no transcript at {}: record it with `OSIRIS_GOLDEN=record just test golden`",
            path.display()
        );
    };
    let recorded: Value = serde_json::from_str(&text).expect("a transcript's JSON");
    let found = contains(&recorded, actual);
    assert!(
        found.is_empty(),
        "{release}'s {scenario} no longer does what was recorded in {}:\n{}\n\nA stable release's \
         behaviour never changes; a preview's transcripts are recorded again with \
         `OSIRIS_GOLDEN=record just test golden`.",
        path.display(),
        found.join("\n")
    );
}
