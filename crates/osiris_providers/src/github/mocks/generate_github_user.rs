//! An account, as GitHub answers it inside a resource and at `GET /user`.

use fake::{
    Fake,
    faker::{internet::en::Username, name::en::Name},
};
use serde_json::{Value, json};

/// An account `id`, as another resource names it.
pub fn generate_github_user(id: u64) -> Value {
    let login: String = Username().fake();
    json!({
        "id": id,
        "login": login,
        "avatar_url": format!("https://avatars.githubusercontent.com/u/{id}"),
        "html_url": format!("https://github.com/{login}"),
        "type": "User",
    })
}

/// The authenticated user `id`, with a profile.
pub fn generate_github_current_user(id: u64) -> Value {
    let mut user = generate_github_user(id);
    if let Some(fields) = user.as_object_mut() {
        fields.insert("name".into(), Value::String(Name().fake()));
        fields.insert("email".into(), Value::Null);
        fields.insert("bio".into(), Value::Null);
        fields.insert("company".into(), Value::Null);
        fields.insert("location".into(), Value::Null);
    }
    user
}
