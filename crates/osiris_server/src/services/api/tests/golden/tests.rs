//! Every release osiris serves has its transcripts replayed.

use osiris_versioning::RELEASES;

/// The names of the releases replayed.
macro_rules! release_names {
    ([$($module:ident $release:literal),* $(,)?]) => {
        const REPLAYED: &[&str] = &[$($release),*];
    };
}

for_every_release!(release_names!());

#[test]
fn replays_every_release_osiris_serves() {
    let mut served: Vec<&str> = RELEASES.iter().map(|release| release.name).collect();
    let mut replayed = REPLAYED.to_vec();
    served.sort_unstable();
    replayed.sort_unstable();
    assert_eq!(
        served, replayed,
        "every release osiris serves gets its line in `for_every_release!` (golden/releases.rs), \
         and its transcripts recorded with `OSIRIS_GOLDEN=record just test golden`"
    );
}
