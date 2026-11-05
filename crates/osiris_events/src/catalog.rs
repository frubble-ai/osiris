//! Every message osiris sends, one module per channel.

pub mod events;

use serde::{Deserialize, Serialize};

/// Where a message goes: each channel is a topic.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Channel {
    Events,
}

impl Channel {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Events => "events",
        }
    }
}

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
