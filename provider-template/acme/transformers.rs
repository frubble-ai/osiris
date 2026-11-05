//! Acme's answers as it sends them, and into our types. Only the fields we read.

use super::types::AcmeWidget;
use serde::Deserialize;

#[derive(Deserialize)]
pub struct WidgetsPage {
    pub widgets: Vec<WidgetAnswer>,
    pub next: Option<String>,
}

#[derive(Deserialize)]
pub struct WidgetAnswer {
    id: u64,
    title: String,
}

impl From<WidgetAnswer> for AcmeWidget {
    fn from(answer: WidgetAnswer) -> Self {
        Self { id: answer.id.to_string(), name: answer.title }
    }
}
