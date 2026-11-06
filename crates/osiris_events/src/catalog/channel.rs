//! Where a message goes: each channel is a topic.

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
