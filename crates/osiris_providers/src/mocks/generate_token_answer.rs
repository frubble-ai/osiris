//! A provider's token answer, with random tokens.

use fake::{Fake, faker::lorem::en::Word};
use serde_json::{Value, json};

/// An answer with an access token, a refresh token, and an access token expiring in `expires_in`
/// seconds (`None`: it doesn't expire).
pub fn generate_token_answer(expires_in: Option<i64>) -> Value {
    let mut answer = json!({
        "access_token": format!("access-{}", 24.fake::<String>()),
        "refresh_token": format!("refresh-{}", 24.fake::<String>()),
        "scope": Word().fake::<String>(),
    });
    if let (Some(seconds), Some(fields)) = (expires_in, answer.as_object_mut()) {
        fields.insert("expires_in".into(), seconds.into());
    }
    answer
}

/// The same answer without a refresh token.
pub fn generate_token_answer_without_refresh(expires_in: Option<i64>) -> Value {
    let mut answer = generate_token_answer(expires_in);
    if let Some(fields) = answer.as_object_mut() {
        fields.remove("refresh_token");
    }
    answer
}
