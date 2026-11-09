//! How osiris finds this provider: its id, where it is by default, and how it is built.

use super::Github;
use crate::registration::{Registered, Registration};

pub static REGISTRATION: Registration = Registration {
    id: "github",
    oauth_base_url: "https://github.com",
    api_base_url: "https://api.github.com",
    build: |urls, limits| Ok(Registered::new(Github::new(urls, limits)?)),
};
