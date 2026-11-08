//! `acme.list_widgets`: every widget the user can see, across pages, up to `max_items`.

use crate::{
    ProviderError,
    acme::{Acme, transformers::WidgetsPage, types::AcmeWidget},
};
use osiris_interfaces::call::{Request, call};
use reqwest::Method;
use serde::Deserialize;
use utoipa::ToSchema;

/// No input: every widget the connection's user can see.
#[derive(Debug, Default, Deserialize, ToSchema)]
#[serde(deny_unknown_fields)]
pub struct ListWidgets {}

impl ListWidgets {
    /// The scopes it needs, as the provider names them.
    pub const SCOPES: &[&str] = &["widgets:read"];
}

impl Acme {
    pub async fn list_widgets(&self, http: &reqwest::Client, token: &str, _input: &ListWidgets) -> Result<Vec<AcmeWidget>, ProviderError> {
        let mut widgets = Vec::new();
        let mut cursor = String::new();
        loop {
            let query = [("cursor", cursor.clone())];
            let page: WidgetsPage = call::<WidgetsPage, ()>(self, http, token, Request { method: Method::GET, path: "/widgets", query: &query, headers: &[], body: None }).await?;
            widgets.extend(page.widgets.into_iter().map(AcmeWidget::from));
            if widgets.len() >= self.limits.max_items {
                widgets.truncate(self.limits.max_items);
                return Ok(widgets);
            }
            match page.next {
                Some(next) => cursor = next,
                None => return Ok(widgets),
            }
        }
    }
}
