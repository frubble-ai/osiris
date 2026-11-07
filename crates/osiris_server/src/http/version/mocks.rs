//! A pretend history for the version middleware's tests: an older release, amun, whose auth
//! configs called `client_id` `app_id`, and anubis as the release that renamed it.

pub mod mock_renamed_client_id;

pub use mock_renamed_client_id::{MOCK_RELEASES, MockRenamedClientId};
