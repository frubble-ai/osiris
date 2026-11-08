//! How a test adds the fake provider to the ones it builds.

use super::{FAKE_ID, FakeProvider};
use crate::registration::{Registered, Registration};

pub static FAKE_REGISTRATION: Registration = Registration {
    id: FAKE_ID,
    oauth_base_url: "https://fake.invalid",
    api_base_url: "https://api.fake.invalid",
    build: |urls, limits| Ok(Registered::new(FakeProvider::new(urls, limits)?)),
};
