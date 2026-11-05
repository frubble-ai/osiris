//! A message of the catalog, and the channel it goes on.

use super::{Channel, events};
use serde::{Deserialize, Serialize};

/// A message of the catalog.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum Message {
    Event(events::EventMessage),
}

impl Message {
    pub fn channel(&self) -> Channel {
        match self {
            Self::Event(_) => Channel::Events,
        }
    }

    /// The message's type, also sent as the `type` attribute so subscribers can filter.
    pub fn message_type(&self) -> &'static str {
        match self {
            Self::Event(event) => event.event_type.as_str(),
        }
    }
}
