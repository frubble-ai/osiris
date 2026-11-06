//! The action routes, one per action.

use super::{github, linear, slack};
use crate::http::state::ApiState;
use utoipa_axum::{router::OpenApiRouter, routes};

pub fn router() -> OpenApiRouter<ApiState> {
    OpenApiRouter::new()
        .routes(routes!(
            github::list_installations::github_list_installations
        ))
        .routes(routes!(github::list_repositories::github_list_repositories))
        .routes(routes!(linear::list_teams::linear_list_teams))
        .routes(routes!(slack::list_channels::slack_list_channels))
}
