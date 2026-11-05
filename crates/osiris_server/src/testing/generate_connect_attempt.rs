//! A client's connect attempt: who the connection is for, its alias, and where the browser goes
//! back, all random.

use fake::{Fake, faker::internet::en::DomainSuffix, faker::lorem::en::Word};
use serde_json::{Value, json};

/// The body of `POST /v1/connected-accounts` through the app `auth_config_id`.
pub fn generate_connect_attempt(auth_config_id: &str) -> Value {
    json!({
        "auth_config_id": auth_config_id,
        "user_id": format!("org_{}", 16.fake::<String>().replace(|character: char| !character.is_ascii_alphanumeric(), "x")),
        "alias": format!("attempt-{}", Word().fake::<String>()),
        "callback_url": format!(
            "https://{}.{}/integrations/callback?connection={}",
            Word().fake::<String>(),
            DomainSuffix().fake::<String>(),
            Word().fake::<String>()
        ),
    })
}
