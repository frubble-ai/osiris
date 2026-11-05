//! Every message osiris sends, one module per channel.

pub mod channel;
pub mod events;
pub mod message;

pub use channel::Channel;
pub use message::Message;
