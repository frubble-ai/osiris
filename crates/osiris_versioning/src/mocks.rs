//! A pretend history for the translation's tests: changes that each leave their mark, so the order
//! they ran in can be read back.

pub mod mock_trail;

pub use mock_trail::{MOCK_RELEASES, MockTrail};
