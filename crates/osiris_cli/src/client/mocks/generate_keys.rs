//! The API and admin keys of a profile, random.

use fake::{Fake, faker::internet::en::Password};

pub struct Keys {
    pub api: String,
    pub admin: String,
}

pub fn generate_keys() -> Keys {
    Keys {
        api: Password(32..40).fake(),
        admin: Password(32..40).fake(),
    }
}
