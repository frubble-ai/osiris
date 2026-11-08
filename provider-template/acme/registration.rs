//! How osiris finds this provider: its id, where it is by default, and how it is built. `build.rs`
//! lists every folder that has one of these.

use super::Acme;
use crate::registration::{Registered, Registration};

pub static REGISTRATION: Registration = Registration {
    id: "acme",
    oauth_base_url: "https://acme.example",
    api_base_url: "https://api.acme.example",
    build: |urls, limits| Ok(Registered::new(Acme::new(urls, limits)?)),
};
