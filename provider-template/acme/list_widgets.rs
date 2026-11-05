//! `acme.list_widgets`: every widget the user can see, across pages.

use super::{Acme, transformers::WidgetsPage, types::AcmeWidget};
use crate::ProviderError;
use osiris_interfaces::call::{Request, call};
use reqwest::Method;

impl Acme {
    pub async fn list_widgets(&self, http: &reqwest::Client, token: &str) -> Result<Vec<AcmeWidget>, ProviderError> {
        let mut widgets = Vec::new();
        let mut cursor = String::new();
        loop {
            let query = [("cursor", cursor.clone())];
            let page: WidgetsPage = call::<WidgetsPage, ()>(self, http, token, Request { method: Method::GET, path: "/widgets", query: &query, headers: &[], body: None }).await?;
            widgets.extend(page.widgets.into_iter().map(AcmeWidget::from));
            match page.next {
                Some(next) => cursor = next,
                None => return Ok(widgets),
            }
        }
    }
}
