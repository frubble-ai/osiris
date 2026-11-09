//! How osiris finds this provider: its id, where it is by default, and how it is built.

use super::Slack;
use crate::registration::{Registered, Registration};

pub static REGISTRATION: Registration = Registration {
    id: "slack",
    oauth_base_url: "https://slack.com",
    // Ends with `/`: Slack's methods join onto it.
    api_base_url: "https://slack.com/api/",
    build: |urls, limits| Ok(Registered::new(Slack::new(urls, limits)?)),
};
