//! A team, as Linear's GraphQL API answers it.

use fake::{Fake, faker::company::en::Buzzword};
use serde_json::{Value, json};

pub fn generate_linear_team() -> Value {
    json!({ "id": format!("team-{}", 12.fake::<String>()), "name": Buzzword().fake::<String>() })
}
