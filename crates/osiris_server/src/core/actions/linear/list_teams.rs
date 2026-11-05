//! `linear.list_teams`: the Linear teams the connected user can see.

use crate::core::{Core, CoreResult};
use osiris_common::secret::ExposeSecret;
use osiris_providers::linear::types::LinearTeam;

impl Core {
    pub async fn linear_list_teams(&self, id: &str) -> CoreResult<Vec<LinearTeam>> {
        let (provider, http) = (&self.providers.linear, &self.http);
        self.with_token(id, "linear", "linear.list_teams", move |token| async move {
            provider.list_teams(http, token.expose_secret()).await
        })
        .await
    }
}
