//! Which build is running: the commit the image was built from, which the Dockerfile copies from
//! its `OSIRIS_COMMIT` build argument into the `OSIRIS_COMMIT` variable. `/health` reports it, so a
//! rollout can check that the new revision is the one answering. A build without it reports
//! `local`.

use std::sync::OnceLock;

pub fn commit() -> &'static str {
    static COMMIT: OnceLock<String> = OnceLock::new();
    COMMIT.get_or_init(|| {
        std::env::var("OSIRIS_COMMIT")
            .ok()
            .filter(|commit| !commit.trim().is_empty())
            .unwrap_or_else(|| "local".to_owned())
    })
}
