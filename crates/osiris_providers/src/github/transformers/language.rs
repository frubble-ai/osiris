//! A language a repository is written in, by bytes of code, as GitHub answers it.

use crate::github::types::GithubLanguage;
use serde::Deserialize;

#[derive(Deserialize)]
pub struct LanguageAnswer {
    #[serde(default)]
    pub name: String,
    #[serde(default)]
    pub bytes: i64,
}

impl From<LanguageAnswer> for GithubLanguage {
    /// A language a repository is written in, by bytes of code, in our terms.
    fn from(answer: LanguageAnswer) -> Self {
        Self {
            name: answer.name,
            bytes: answer.bytes,
        }
    }
}
