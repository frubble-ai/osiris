//! A language a repository is written in, by bytes of code, as GitHub answers it.

use crate::github::types::GithubLanguage;
use serde::Deserialize;
use std::collections::BTreeMap;

/// `GET /repos/{owner}/{repo}/languages`: each language's name and its bytes of code.
#[derive(Deserialize)]
pub struct LanguagesAnswer(pub BTreeMap<String, i64>);

impl LanguagesAnswer {
    /// Each language in our terms, the most used first, as GitHub orders them.
    pub fn into_languages(self) -> Vec<GithubLanguage> {
        let mut languages: Vec<GithubLanguage> = self
            .0
            .into_iter()
            .map(|(name, bytes)| GithubLanguage { name, bytes })
            .collect();
        languages.sort_by_key(|language| std::cmp::Reverse(language.bytes));
        languages
    }
}

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
