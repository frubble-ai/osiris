//! The names a GitHub request is made with, generated: an account (owner or user), a repository
//! and a token. A test reads the values it generated, so its routes are built from them.

use fake::{Fake, faker::lorem::en::Word};

/// An account's login: letters, digits and a hyphen, as GitHub takes them.
pub fn generate_github_login() -> String {
    format!(
        "{}-{}",
        Word().fake::<String>().to_ascii_lowercase(),
        (1..10_000).fake::<u32>()
    )
}

/// A repository's name: letters, digits, a hyphen and a dot.
pub fn generate_github_repo() -> String {
    format!(
        "{}.{}",
        Word().fake::<String>().to_ascii_lowercase(),
        (1..10_000).fake::<u32>()
    )
}

/// A user token, shaped as GitHub's are.
pub fn generate_github_token() -> String {
    format!(
        "gho_{}",
        36.fake::<String>()
            .replace(|character: char| !character.is_ascii_alphanumeric(), "x")
    )
}
