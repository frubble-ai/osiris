//! Every release osiris serves, with its golden transcripts replayed. A release added to
//! `RELEASES` gets its line here; `tests.rs` fails until it does.

/// Hands `$callback` the releases replayed, each as the name of its test and the release's name.
macro_rules! for_every_release {
    ($callback:ident!($($args:tt)*)) => {
        $callback!($($args)* [anubis "2026-11-04.anubis"]);
    };
}
