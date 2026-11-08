//! Golden transcripts: a scenario run the way a client runs it, at one release, its requests,
//! answers and events written to `api-reference/golden/<release>/<scenario>.json` with ids and
//! times masked. Every build replays each release's transcripts and fails when what osiris does
//! no longer contains what was recorded, which catches the changes no spec shows (a default, an
//! error code, an ordering, a status). Answers may gain fields; nothing recorded may change.
//!
//! `OSIRIS_GOLDEN=record just test golden` writes the transcripts again. CI refuses a pull request
//! that changes a stable release's transcripts.

pub mod contains;
pub mod golden_session;
pub mod masker;
pub mod transcript_file;

pub use golden_session::{GoldenSession, Key};
