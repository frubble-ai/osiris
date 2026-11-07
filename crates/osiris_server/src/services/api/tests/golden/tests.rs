//! Every release has every scenario's transcript.

use super::scenarios::SCENARIOS;
use crate::testing::golden::transcript_file::transcript_path;
use osiris_api_models::version::RELEASES;

#[test]
fn keeps_every_scenario_s_transcript_for_every_release() {
    for release in RELEASES {
        for scenario in SCENARIOS {
            let path = transcript_path(release.name, scenario);
            assert!(
                path.exists() || std::env::var("OSIRIS_GOLDEN").as_deref() == Ok("record"),
                "{release:?} has no {scenario} transcript at {}: add `golden_at!` for it and \
                 record it with `OSIRIS_GOLDEN=record just test golden`",
                path.display()
            );
        }
    }
}
