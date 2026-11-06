//! A channel, as Slack's `conversations.list` answers it.

use fake::{Fake, faker::lorem::en::Word};
use serde_json::{Value, json};

pub fn generate_slack_channel() -> Value {
    json!({
        "id": format!("C{}", 10.fake::<String>().to_uppercase()),
        "name": Word().fake::<String>(),
        "is_private": (0..2).fake::<u8>() == 1,
        "num_members": (1..500).fake::<u32>(),
    })
}
