//! The action routes, one per action.

use super::{github, linear, slack};
use crate::http::state::ApiState;
use utoipa_axum::{router::OpenApiRouter, routes};

pub fn router() -> OpenApiRouter<ApiState> {
    OpenApiRouter::new()
        .merge(github::router())
        .routes(routes!(linear::list_teams::linear_list_teams))
        .routes(routes!(slack::list_channels::slack_list_channels))
}
