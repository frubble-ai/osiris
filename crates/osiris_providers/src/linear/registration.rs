//! How osiris finds this provider: its id, where it is by default, and how it is built.

use super::Linear;
use crate::registration::{Registered, Registration};

pub static REGISTRATION: Registration = Registration {
    id: "linear",
    oauth_base_url: "https://linear.app",
    api_base_url: "https://api.linear.app",
    build: |urls, limits| Ok(Registered::new(Linear::new(urls, limits)?)),
};
