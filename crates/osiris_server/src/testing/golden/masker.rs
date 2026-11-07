//! What changes from run to run, replaced by stable placeholders: osiris's ids (`ac_…`, `ca_…`)
//! numbered by first sight, so the same id is the same placeholder across a transcript; times; and
//! long random tokens (a consent's `state`, its PKCE challenge); and the faked providers' origin,
//! whose port changes every run.

use serde_json::Value;
use std::collections::HashMap;
use time::{OffsetDateTime, format_description::well_known::Rfc3339};

#[derive(Default)]
pub struct Masker {
    origin: String,
    ids: HashMap<String, String>,
    counts: HashMap<String, usize>,
}

impl Masker {
    /// A masker that writes the faked providers' `origin` as `<provider>`.
    pub fn new(origin: &str) -> Self {
        Self {
            origin: origin.trim_end_matches('/').to_owned(),
            ..Self::default()
        }
    }

    pub fn mask(&mut self, value: &Value) -> Value {
        match value {
            Value::String(text) => Value::String(self.mask_text(text)),
            Value::Array(items) => Value::Array(items.iter().map(|item| self.mask(item)).collect()),
            Value::Object(map) => Value::Object(
                map.iter()
                    .map(|(key, child)| (key.clone(), self.mask(child)))
                    .collect(),
            ),
            other => other.clone(),
        }
    }

    pub fn mask_text(&mut self, text: &str) -> String {
        if OffsetDateTime::parse(text, &Rfc3339).is_ok() {
            return "<time>".into();
        }
        let text = if self.origin.is_empty() {
            text.to_owned()
        } else {
            text.replace(&self.origin, "<provider>")
        };
        let mut masked = String::with_capacity(text.len());
        let mut word = String::new();
        for character in text.chars() {
            if character.is_ascii_alphanumeric() || character == '_' || character == '-' {
                word.push(character);
            } else {
                masked.push_str(&self.mask_word(&std::mem::take(&mut word)));
                masked.push(character);
            }
        }
        masked.push_str(&self.mask_word(&word));
        masked
    }

    fn mask_word(&mut self, word: &str) -> String {
        if let Some((prefix, rest)) = word.split_once('_')
            && (2..=5).contains(&prefix.len())
            && prefix
                .chars()
                .all(|character| character.is_ascii_lowercase())
            && rest.len() >= 16
            && rest
                .chars()
                .all(|character| character.is_ascii_alphanumeric())
        {
            if let Some(placeholder) = self.ids.get(word) {
                return placeholder.clone();
            }
            let count = self.counts.entry(prefix.to_owned()).or_default();
            *count += 1;
            let placeholder = format!("<{prefix}_{count}>");
            self.ids.insert(word.to_owned(), placeholder.clone());
            return placeholder;
        }
        if word.len() >= 24 {
            return "<random>".into();
        }
        word.to_owned()
    }
}

#[cfg(test)]
mod tests;
