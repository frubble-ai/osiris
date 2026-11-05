//! `linear.list_teams`: the teams the connected user can see. The query as in `OpenConnector` v1.9.4
//! (Apache-2.0), `src/providers/linear/executors.ts`; 100 a page is Linear's own maximum.

use super::{Linear, graphql::graphql, transformers::TeamsData, types::LinearTeam};
use crate::ProviderError;
use serde_json::json;

const QUERY: &str = "query Teams($after: String) { teams(after: $after, first: 100, includeArchived: false) { nodes { id name } pageInfo { hasNextPage endCursor } } }";

impl Linear {
    /// Every team, archived ones left out, across pages.
    pub async fn list_teams(
        &self,
        http: &reqwest::Client,
        token: &str,
    ) -> Result<Vec<LinearTeam>, ProviderError> {
        let mut teams = Vec::new();
        let mut after: Option<String> = None;
        loop {
            let page: TeamsData =
                graphql(self, http, token, QUERY, json!({ "after": after })).await?;
            let more = page.teams.page_info.has_next_page;
            after.clone_from(&page.teams.page_info.end_cursor);
            teams.extend(page.teams.teams());
            if !more {
                return Ok(teams);
            }
        }
    }
}
